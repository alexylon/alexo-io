use crate::components::data::LANGUAGES;
use dioxus::prelude::*;

#[component]
pub fn LanguagesSection() -> Element {
    rsx! {
        section {
            class: "languages-section section",
            h2 { "Languages" }
            div {
                class: "languages-grid",
                {LANGUAGES.iter().map(|lang| rsx! {
                    span {
                        class: "language-item",
                        span { "{lang.name}" }
                        span {
                            class: "language-level",
                            "{lang.level}"
                        }
                    }
                })}
            }
        }
    }
}
