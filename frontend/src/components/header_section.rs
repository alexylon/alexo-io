use crate::components::data::PROFILE;
use dioxus::prelude::*;

#[component]
pub fn HeaderSection() -> Element {
    let now = js_sys::Date::new_0();
    let years = now.get_full_year().saturating_sub(2019) - if now.get_month() < 8 { 1 } else { 0 };

    rsx! {
        section {
            class: "header-section section",
            span {
                class: "hero-eyebrow reveal",
                "{PROFILE.title}"
            }
            h1 {
                class: "reveal",
                "{PROFILE.name}"
            }
            p {
                class: "hero-meta reveal",
                "Sofia, Bulgaria \u{2002}/\u{2002} {years}+ years"
            }
        }
    }
}
