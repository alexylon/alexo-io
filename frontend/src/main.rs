use dioxus::prelude::*;
use manganis::Asset;
use std::rc::Rc;

mod components;
mod theme_store;
use components::*;

fn main() {
    LaunchBuilder::new().launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Dark,
    Light,
}

impl Theme {
    fn css_class(&self) -> &'static str {
        match self {
            Theme::Dark => "theme-dark",
            Theme::Light => "theme-light",
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

pub(crate) fn preferred_scroll_behavior() -> ScrollBehavior {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(mq)) = window.match_media("(prefers-reduced-motion: reduce)") {
            if mq.matches() {
                return ScrollBehavior::Instant;
            }
        }
    }
    ScrollBehavior::Smooth
}

fn get_system_theme() -> Theme {
    use web_sys::window;

    if let Some(window) = window() {
        if let Ok(media_query) = window.match_media("(prefers-color-scheme: dark)") {
            if let Some(media_query) = media_query {
                if media_query.matches() {
                    return Theme::Dark;
                }
            }
        }
    }

    Theme::Light
}

#[component]
fn App() -> Element {
    let mut theme = use_signal(|| get_system_theme());

    // Load persisted theme, then reveal the page
    use_effect(move || {
        spawn(async move {
            if let Some(saved) = theme_store::load_theme().await {
                theme.set(saved);
            }
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        let _ = body.set_attribute("class", "ready");
                    }
                }
            }
        });
    });

    let mut top_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let skills_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let experience_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let projects_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let education_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let contact_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let active_section: Signal<String> = use_signal(String::new);

    let font_css = format!(
        r#"
        @font-face {{
            font-family: 'Atkinson Hyperlegible Next';
            src: url('{}') format('opentype');
            font-weight: 400;
            font-style: normal;
            font-display: swap;
        }}
        @font-face {{
            font-family: 'Podkova';
            src: url('{}') format('truetype');
            font-weight: 700;
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
        asset!("/assets/fonts/AtkinsonHyperlegibleNext-Regular.otf"),
        asset!("/assets/fonts/Podkova-Bold.ttf"),
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
            NavSection { theme, active_section, skills_section, experience_section, projects_section, education_section, contact_section }
            div {
                class: "resume",
                onmounted: move |cx| top_element.set(Some(cx.data())),
                header {
                    class: "hero",
                    HeaderSection {}
                    AboutSection {}
                }
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
