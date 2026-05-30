use crate::components::data::CONTACT_LINKS;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn ContactSection(contact_section: Signal<Option<Rc<MountedData>>>) -> Element {
    let resume_href = asset!("/assets/docs/Resume_Alexander_Alexandrov.pdf").to_string();

    rsx! {
        section {
            id: "contact",
            onmounted: move |cx| contact_section.set(Some(cx.data())),
            class: "contact-section section",
            h2 { "Contact" }
            p {
                class: "contact-intro",
                "Always happy to talk software design, Rust, or wine."
            }
            div {
                class: "contact-grid",
                {CONTACT_LINKS.iter().map(|link| {
                    let css_class = format!(
                        "contact-card contact-{}",
                        link.label.split_whitespace().next().unwrap_or("").to_lowercase()
                    );
                    let href = if link.download.is_some() {
                        resume_href.clone()
                    } else {
                        link.href.to_string()
                    };
                    rsx! {
                        a {
                            class: "{css_class}",
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
