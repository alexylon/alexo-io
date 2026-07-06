use dioxus::prelude::*;
use manganis::Asset;
use std::rc::Rc;

mod components;
mod theme_store;
use components::*;

fn main() {
    dioxus::LaunchBuilder::new()
        // server_only! compiles to () on the web build. On the server build it
        // sets up incremental rendering into public/, which is what `--ssg` uses
        // to pre-render the routes.
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

/// At build time the CLI calls this to learn which routes to pre-render. The
/// endpoint name must be exactly `static_routes` — that's what the CLI looks for.
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
    /// localStorage key for the saved choice. prerender.sh's pre-paint script
    /// reads the same key — keep them in sync.
    #[cfg(target_arch = "wasm32")]
    const STORAGE_KEY: &'static str = "theme";

    fn css_class(&self) -> &'static str {
        match self {
            Theme::Dark => "theme-dark",
            Theme::Light => "theme-light",
        }
    }

    /// Inverse of `from_storage_value` — change both together, and mirror the
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

    fn icon_theme(&self) -> Asset {
        match self {
            Theme::Dark => asset!("/assets/icons/light_mode.svg"),
            Theme::Light => asset!("/assets/icons/dark_mode.svg"),
        }
    }

    fn icon_up(&self) -> Asset {
        match self {
            Theme::Dark => asset!("/assets/icons/keyboard_arrow_up_light.svg"),
            Theme::Light => asset!("/assets/icons/keyboard_arrow_up_dark.svg"),
        }
    }
}

/// Smooth scroll unless the user prefers reduced motion (always Smooth on the
/// server build, where only the client scrolls).
pub(crate) fn preferred_scroll_behavior() -> ScrollBehavior {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(mq)) = window.match_media("(prefers-reduced-motion: reduce)") {
                if mq.matches() {
                    return ScrollBehavior::Instant;
                }
            }
        }
    }
    ScrollBehavior::Smooth
}

/// Today's `(year, month0)`, month 0-indexed. Computed per-target so the
/// server-prerendered value matches the client's on hydration.
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

        // Walk forward from the epoch year-by-year then month-by-month, to
        // avoid a chrono dependency.
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

    // After hydration, switch from the default to the saved/OS theme. Running
    // here (not at first render) is what keeps server and client in sync.
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(t) = theme_store::resolve_theme() {
                theme.set(t);
            }
        }
    });

    let mut top_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let skills_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let experience_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let projects_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let education_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let contact_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let active_section: Signal<String> = use_signal(String::new);

    // Variable fonts, split into latin + cyrillic subsets; the browser fetches
    // per unicode-range.
    let font_css = format!(
        r#"
        @font-face {{
            font-family: 'Literata';
            src: url('{}') format('woff2');
            font-weight: 200 900;
            font-style: normal;
            font-display: swap;
            unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
        }}
        @font-face {{
            font-family: 'Literata';
            src: url('{}') format('woff2');
            font-weight: 200 900;
            font-style: normal;
            font-display: swap;
            unicode-range: U+0301, U+0400-045F, U+0490-0491, U+04B0-04B1, U+2116;
        }}
        @font-face {{
            font-family: 'Literata';
            src: url('{}') format('woff2');
            font-weight: 200 900;
            font-style: italic;
            font-display: swap;
            unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
        }}
        @font-face {{
            font-family: 'Literata';
            src: url('{}') format('woff2');
            font-weight: 200 900;
            font-style: italic;
            font-display: swap;
            unicode-range: U+0301, U+0400-045F, U+0490-0491, U+04B0-04B1, U+2116;
        }}
        @font-face {{
            font-family: 'IBM Plex Sans';
            src: url('{}') format('woff2');
            font-weight: 100 700;
            font-style: normal;
            font-display: swap;
        }}
        @font-face {{
            font-family: 'Atkinson Hyperlegible Mono';
            src: url('{}') format('opentype');
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
        asset!("/assets/fonts/AtkinsonHyperlegibleMono-Regular.otf"),
    );

    rsx! {
        style { {font_css} }
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
            NavSection { theme, active_section, top_element, skills_section, experience_section, projects_section, education_section, contact_section }
            div {
                class: "resume",
                onmounted: move |cx| top_element.set(Some(cx.data())),
                HeroSection {}
                SkillsSection { skills_section }
                ExperienceSection { experience_section }
                ProjectsSection { projects_section }
                EducationSection { education_section }
                CertificationsSection {}
                LanguagesSection {}
                ContactSection { contact_section }
                FooterSection {}
                ScrollToTop { top_element, theme }
            }
        }
    }
}
