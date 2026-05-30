use crate::components::data::CONTACT_LINKS;
use dioxus::prelude::*;

/// Labels surfaced as quick links in the masthead (the rest live in Contact).
const HERO_LINK_LABELS: &[&str] = &["Email", "GitHub", "LinkedIn", "Mastodon", "Resume"];

#[component]
pub fn AboutSection() -> Element {
    let mut is_image_expanded = use_signal(|| false);
    let resume_href = asset!("/assets/docs/Resume_Alexander_Alexandrov.pdf").to_string();

    rsx! {
        div {
            class: "about-block",
            div {
                class: "about-photo-frame",
                button {
                    r#type: "button",
                    class: "about-photo-button reveal",
                    aria_label: "Expand portrait of Alexander Alexandrov",
                    onclick: move |_| {
                        is_image_expanded.set(true);
                    },
                    img {
                        class: "about-photo",
                        src: asset!("/assets/images/profilepic.jpg"),
                        alt: "Alexander Alexandrov",
                        width: "158",
                        height: "158",
                    }
                }
            }
            div {
                class: "about-text reveal",
                p {
                    "I build web apps, backend services, and developer tools with clean, \
                    readable, performant code \u{2014} using "
                    span { class: "accent", "Rust" }
                    " when it\u{2019}s the right tool."
                }
                div {
                    class: "hero-links",
                    {CONTACT_LINKS.iter()
                        .filter(|link| HERO_LINK_LABELS.iter().any(|l| link.label.starts_with(l)))
                        .map(|link| {
                            let href = if link.download.is_some() {
                                resume_href.clone()
                            } else {
                                link.href.to_string()
                            };
                            rsx! {
                                a {
                                    class: "hero-link",
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

        if is_image_expanded() {
            div {
                class: "image-overlay",
                tabindex: "0",
                onclick: move |_| {
                    is_image_expanded.set(false);
                },
                onkeydown: move |e: KeyboardEvent| {
                    if e.key() == Key::Escape {
                        is_image_expanded.set(false);
                    }
                },
                onmounted: move |cx| async move {
                    let _ = cx.set_focus(true).await;
                },
                button {
                    r#type: "button",
                    class: "close-button",
                    aria_label: "Close portrait",
                    onclick: move |e| {
                        e.stop_propagation();
                        is_image_expanded.set(false);
                    },
                    "\u{00D7}"
                }
                img {
                    src: asset!("/assets/images/profilepic.jpg"),
                    alt: "Alexander Alexandrov",
                    onclick: move |e| {
                        e.stop_propagation();
                    }
                }
            }
        }
    }
}
