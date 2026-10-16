use dioxus::prelude::*;

mod components;
mod theme_store;
use components::*;

fn main() {
    dioxus::LaunchBuilder::new()
        // Compiles to () on the web build. On the server build it sets up the
        // incremental rendering into public/ that `--ssg` pre-renders into.
        .with_cfg(server_only! {
            ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // Render into the `public/` dir next to the server binary.
                        .static_dir(
                            std::env::current_exe()
                                .expect("server binary path is knowable at build time")
                                .parent()
                                .expect("server binary path has a parent directory")
                                .join("public"),
                        )
                        .clear_cache(false),
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

/// The CLI calls this at build time to learn which routes to pre-render. The
/// endpoint name must be exactly `static_routes`.
#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Dark,
    Light,
}

impl Theme {
    /// prerender.sh's pre-paint script reads the same key. Keep them in sync.
    #[cfg(target_arch = "wasm32")]
    const STORAGE_KEY: &'static str = "theme";

    fn css_class(&self) -> &'static str {
        match self {
            Theme::Dark => "theme-dark",
            Theme::Light => "theme-light",
        }
    }

    /// Inverse of `from_storage_value`. Change both together, and mirror the
    /// values in prerender.sh's pre-paint script.
    #[cfg(target_arch = "wasm32")]
    fn storage_value(&self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn from_storage_value(value: &str) -> Option<Self> {
        match value {
            "dark" => Some(Theme::Dark),
            "light" => Some(Theme::Light),
            _ => None,
        }
    }

    fn toggle(&self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }
}

/// Today's `(year, month0)`, month 0-indexed: build time in the prerender, visit
/// time in the browser.
fn current_year_month0() -> (u32, u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let now = js_sys::Date::new_0();
        (now.get_full_year(), now.get_month())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        const SECS_PER_DAY: u64 = 86_400;
        const EPOCH_YEAR: i64 = 1970;

        let is_leap = |y: i64| (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;

        // Walk forward from the epoch rather than pull in chrono.
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let mut days = (secs / SECS_PER_DAY) as i64;

        let mut year = EPOCH_YEAR;
        loop {
            let days_in_year = if is_leap(year) { 366 } else { 365 };
            if days < days_in_year {
                break;
            }
            days -= days_in_year;
            year += 1;
        }

        let feb = if is_leap(year) { 29 } else { 28 };
        let month_lengths = [31, feb, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut month0 = 0u32;
        for len in month_lengths {
            if days < len {
                break;
            }
            days -= len;
            month0 += 1;
        }

        (year as u32, month0)
    }
}

pub(crate) fn current_year() -> u32 {
    current_year_month0().0
}

/// Starts at the prerendered value so hydration matches the page, then moves to
/// the visitor's date: hydration alone would keep the build-time text.
#[track_caller]
pub(crate) fn use_current(now: fn() -> u32) -> Signal<u32> {
    let built = use_server_cached(now);
    let mut value = use_signal(|| built);
    use_effect(move || value.set(now()));
    value
}

/// Whole years since `start_year`/`start_month0`, counting a year only once its
/// anniversary month is reached. Saturates at 0 for a future start date.
pub(crate) fn years_since(start_year: u32, start_month0: u32) -> u32 {
    let (year, month0) = current_year_month0();
    let mut elapsed = year.saturating_sub(start_year);
    if month0 < start_month0 {
        elapsed = elapsed.saturating_sub(1);
    }
    elapsed
}

/// Fixed default so the server render and the client's first render agree (no
/// hydration mismatch); the client switches to the resolved theme just after.
fn initial_theme() -> Theme {
    Theme::Light
}

#[component]
fn Home() -> Element {
    #[cfg_attr(not(target_arch = "wasm32"), allow(unused_mut))]
    let mut theme = use_signal(initial_theme);

    // Runs after hydration, not at first render, so server and client agree.
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(t) = theme_store::resolve_theme() {
                theme.set(t);
            }
        }
    });

    let active_section: Signal<String> = use_signal(String::new);

    rsx! {
        FontFaces {}
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/styling/index.css")
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/styling/theme-dark.css"),
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/styling/theme-light.css"),
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            href: asset!("/assets/images/favicon.png"),
        }
        document::Link {
            rel: "apple-touch-icon",
            href: asset!("/assets/images/apple-touch-icon.png"),
        }
        main {
            class: "{theme().css_class()}",
            a {
                class: "skip-link",
                href: "#top",
                onclick: move |evt: MouseEvent| {
                    if !evt.modifiers().is_empty() {
                        return;
                    }
                    evt.prevent_default();
                    components::go_to_top();
                },
                "Skip to content"
            }
            NavSection { theme, active_section }
            div {
                class: "resume",
                // Target for the skip link, wordmark and scroll-to-top.
                // tabindex makes it a real focus destination.
                id: "top",
                tabindex: "-1",
                HeroSection {}
                // Evidence before inventory: the skills list must not stand
                // between the hero and the first named system.
                ExperienceSection {}
                ProjectsSection {}
                SkillsSection {}
                EducationSection {}
                CertificationsSection {}
                LanguagesSection {}
                ContactSection {}
                FooterSection {}
                ScrollToTop {}
            }
        }
    }
}

/// Props-less so Dioxus memoises it. Inside `Home` it would rebuild on every
/// theme toggle, and `document::Style` warns on the console whenever its props
/// change after the first render.
#[component]
fn FontFaces() -> Element {
    // Split into latin and cyrillic subsets; the browser fetches per
    // unicode-range. Weight axes are pinned to the 400-600 the design uses.
    let font_css = format!(
        r#"
        @font-face {{
            font-family: 'Literata';
            src: url('{}') format('woff2');
            font-weight: 400 600;
            font-style: normal;
            font-display: swap;
            unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
        }}
        @font-face {{
            font-family: 'Literata';
            src: url('{}') format('woff2');
            font-weight: 400 600;
            font-style: normal;
            font-display: swap;
            unicode-range: U+0301, U+0400-045F, U+0490-0491, U+04B0-04B1, U+2116;
        }}
        @font-face {{
            font-family: 'Literata';
            src: url('{}') format('woff2');
            font-weight: 400 600;
            font-style: italic;
            font-display: swap;
            unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
        }}
        @font-face {{
            font-family: 'Literata';
            src: url('{}') format('woff2');
            font-weight: 400 600;
            font-style: italic;
            font-display: swap;
            unicode-range: U+0301, U+0400-045F, U+0490-0491, U+04B0-04B1, U+2116;
        }}
        @font-face {{
            font-family: 'IBM Plex Sans';
            src: url('{}') format('woff2');
            font-weight: 400;
            font-style: normal;
            font-display: swap;
        }}
        @font-face {{
            font-family: 'Atkinson Hyperlegible Mono';
            src: url('{}') format('woff2');
            font-weight: 400;
            font-style: normal;
            font-display: swap;
        }}
        "#,
        asset!("/assets/fonts/Literata-Latin.woff2"),
        asset!("/assets/fonts/Literata-Cyrillic.woff2"),
        asset!("/assets/fonts/Literata-Italic-Latin.woff2"),
        asset!("/assets/fonts/Literata-Italic-Cyrillic.woff2"),
        asset!("/assets/fonts/IBMPlexSans-Latin.woff2"),
        asset!("/assets/fonts/AtkinsonHyperlegibleMono-Regular.woff2"),
    );

    rsx! {
        // Must be `document::Style`, never a bare `style {}`. Dioxus puts a
        // hydration marker at the start of a vdom node, and inside a <style>
        // that marker parses as CSS and swallows the first @font-face rule.
        document::Style { {font_css} }
    }
}
