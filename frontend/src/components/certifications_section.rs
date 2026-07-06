use crate::components::data::CERTIFICATIONS;
use crate::components::entry_card::EntryCard;
use dioxus::prelude::*;

#[component]
pub fn CertificationsSection() -> Element {
    rsx! {
        section {
            class: "certification-section section",
            h2 { "Certification" }
            div {
                class: "entry-list entry-list-tight",
                {CERTIFICATIONS.iter().map(|cert| rsx! {
                    EntryCard {
                        apparatus: rsx! { "{cert.meta}" },
                        title: rsx! {
                            a {
                                href: "{cert.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "{cert.title}"
                            }
                        },
                    }
                })}
            }
        }
    }
}
