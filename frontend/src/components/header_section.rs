use crate::components::data::PROFILE;
use dioxus::prelude::*;

// Career start: September 2019 (month index 8). Drives the "N+ years" figure.
const CAREER_START_YEAR: u32 = 2019;
const CAREER_START_MONTH0: u32 = 8;

#[component]
pub fn HeaderSection() -> Element {
    let years = crate::years_since(CAREER_START_YEAR, CAREER_START_MONTH0);

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
                "Sofia, Bulgaria \u{2002}/\u{2002} {years}+ years building software"
            }
        }
    }
}
