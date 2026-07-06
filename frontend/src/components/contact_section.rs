use crate::components::data::CONTACT_LINKS;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn ContactSection(contact_section: Signal<Option<Rc<MountedData>>>) -> Element {
    let resume_href = asset!("/assets/docs/Resume_Alexander_Alexandrov.pdf").to_string();
    let email = CONTACT_LINKS
        .iter()
        .find(|link| link.href.starts_with("mailto:"));

    rsx! {
        section {
            id: "contact",
            onmounted: move |cx| contact_section.set(Some(cx.data())),
            class: "contact-section section",
            h2 { "Contact" }
            p {
                class: "contact-intro",
                "Always happy to discuss software design, Rust, or wine."
            }
            if let Some(email) = email {
                a {
                    class: "contact-email",
                    href: "{email.href}",
                    {email.href.trim_start_matches("mailto:")}
                }
            }
            div {
                class: "contact-links",
                {CONTACT_LINKS.iter()
                    .filter(|link| !link.href.starts_with("mailto:"))
                    .map(|link| {
                        let is_resume = link.download.is_some();
                        let href = if is_resume {
                            resume_href.clone()
                        } else {
                            link.href.to_string()
                        };
                        let class = if is_resume { "contact-resume" } else { "contact-link" };
                        rsx! {
                            a {
                                class: "{class}",
                                href: "{href}",
                                target: link.target.unwrap_or(""),
                                rel: link.rel.unwrap_or(""),
                                download: link.download.unwrap_or(""),
                                "{link.label}"
                            }
                        }
                    })}
            }
        }
    }
}
