use crate::components::data::{CONTACT_LINKS, PROFILE};
use dioxus::prelude::*;

// Career start: September 2019 (month index 8). Drives the "N+ years" figure.
const CAREER_START_YEAR: u32 = 2019;
const CAREER_START_MONTH0: u32 = 8;

#[component]
pub fn HeroSection() -> Element {
    let mut is_image_expanded = use_signal(|| false);
    let resume_href = asset!("/assets/docs/Resume_Alexander_Alexandrov.pdf").to_string();
    let years = crate::years_since(CAREER_START_YEAR, CAREER_START_MONTH0);

    rsx! {
        header {
            class: "hero",
            div {
                class: "hero-rail reveal",
                button {
                    r#type: "button",
                    class: "about-photo-button",
                    aria_label: "Expand portrait of Alexander Alexandrov",
                    onclick: move |_| {
                        is_image_expanded.set(true);
                    },
                    img {
                        class: "about-photo",
                        src: asset!("/assets/images/profilepic.jpeg"),
                        alt: "Alexander Alexandrov",
                        width: "320",
                        height: "320",
                    }
                }
                div {
                    class: "hero-docket",
                    span { "Sofia \u{00B7} Bulgaria" }
                    span { "{years}+ years" }
                    span { "BG\u{00B7}EN\u{00B7}IT\u{00B7}RU\u{00B7}EL" }
                }
            }
            div {
                class: "hero-body",
                span {
                    class: "hero-cyrillic reveal",
                    lang: "bg",
                    "Александър Александров"
                }
                h1 {
                    class: "reveal",
                    "{PROFILE.name}"
                }
                p {
                    class: "hero-role reveal",
                    "{PROFILE.title}"
                }
                p {
                    class: "hero-lede reveal",
                    "I build web applications, backend services, and developer tools with clean, \
                    readable, performant code \u{2014} using "
                    span { class: "accent", "Rust" }
                    " when it\u{2019}s the right tool."
                }
                div {
                    class: "hero-links reveal",
                    {CONTACT_LINKS.iter().map(|link| {
                        let href = if link.download.is_some() {
                            resume_href.clone()
                        } else {
                            link.href.to_string()
                        };
                        let (label, class) = if link.href.starts_with("mailto:") {
                            (link.href.trim_start_matches("mailto:"), "hero-link hero-link-email")
                        } else {
                            (link.label, "hero-link")
                        };
                        rsx! {
                            a {
                                class: "{class}",
                                href: "{href}",
                                target: link.target.unwrap_or(""),
                                rel: link.rel.unwrap_or(""),
                                download: link.download.unwrap_or(""),
                                "{label}"
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
                    src: asset!("/assets/images/profilepic.jpeg"),
                    alt: "Alexander Alexandrov",
                    onclick: move |e| {
                        e.stop_propagation();
                    }
                }
            }
        }
    }
}
