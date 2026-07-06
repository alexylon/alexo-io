use crate::components::data::LANGUAGES;
use dioxus::prelude::*;

#[component]
pub fn LanguagesSection() -> Element {
    rsx! {
        section {
            class: "languages-section section",
            h2 { "Spoken Languages" }
            p {
                class: "lang-line",
                {LANGUAGES.iter().enumerate().map(|(i, lang)| rsx! {
                    if i > 0 {
                        span { class: "sep", "\u{00B7}" }
                    }
                    span {
                        "{lang.name}"
                        span {
                            class: "lang-level",
                            "{lang.level}"
                        }
                    }
                })}
            }
        }
    }
}
