use crate::components::data::{CONTACT_LINKS, PROFILE};
use dioxus::prelude::*;

// September 2019, month index 8. Drives the "N+ years" figure.
const CAREER_START_YEAR: u32 = 2019;
const CAREER_START_MONTH0: u32 = 8;

/// The dialog stays mounted; its `open` attribute is the only source of truth.
/// Rendered conditionally, the browser can close it while the app still thinks
/// it is open, leaving an invisible full-screen layer over the page. Staying
/// mounted also lets `show_modal()` and `close()` trap focus and hand it back
/// to the trigger.
#[cfg(target_arch = "wasm32")]
fn portrait_dialog() -> Option<web_sys::HtmlDialogElement> {
    use wasm_bindgen::JsCast;
    web_sys::window()?
        .document()?
        .get_element_by_id("portrait-dialog")?
        .dyn_into()
        .ok()
}

fn open_portrait() {
    #[cfg(target_arch = "wasm32")]
    if let Some(dialog) = portrait_dialog() {
        let _ = dialog.show_modal();
    }
}

fn close_portrait() {
    #[cfg(target_arch = "wasm32")]
    if let Some(dialog) = portrait_dialog() {
        dialog.close();
    }
}

#[component]
pub fn HeroSection() -> Element {
    let resume_href = asset!("/assets/docs/Resume_Alexander_Alexandrov.pdf").to_string();
    let years = crate::years_since(CAREER_START_YEAR, CAREER_START_MONTH0);

    rsx! {
        header {
            class: "hero",
            div {
                class: "hero-rail",
                button {
                    id: "portrait-trigger",
                    r#type: "button",
                    class: "about-photo-button reveal",
                    aria_label: "Expand portrait of Alexander Alexandrov",
                    onclick: move |_| open_portrait(),
                    img {
                        class: "about-photo",
                        src: asset!("/assets/images/profilepic.jpeg"),
                        alt: "Alexander Alexandrov",
                        width: "640",
                        height: "640",
                    }
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
                // A definition to take in at a glance, then the detail. The
                // emphasis belongs on the degrees: the range is the thing worth
                // remembering, not the language.
                p {
                    class: "hero-lede reveal",
                    "I build web applications, backend services, and developer tools."
                }
                p {
                    class: "hero-lede reveal",
                    "Currently working in digital publishing; previously in enterprise \
                    product modelling and automotive security. I publish "
                    // The claim and its proof are one tap apart. A plain anchor,
                    // so it works before WASM loads; once loaded it makes the
                    // nav's move, without a history entry.
                    a {
                        class: "hero-lede-link",
                        href: "#projects",
                        onclick: move |evt: MouseEvent| {
                            if !evt.modifiers().is_empty() {
                                return;
                            }
                            evt.prevent_default();
                            crate::components::go_to_section("projects");
                        },
                        "open-source Rust tools"
                    }
                    " on crates.io and hold "
                    span { class: "accent", "an MEng in Engineering and a PhD in Theology" }
                    "."
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
                        } else if link.download.is_some() {
                            // Marked so it does not read as another profile link.
                            (link.label, "hero-link hero-link-file")
                        } else {
                            (link.label, "hero-link")
                        };
                        rsx! {
                            // Optional, never `unwrap_or("")`: an empty
                            // `download` attribute still counts as one, and
                            // would mark every link as a download.
                            a {
                                class: "{class}",
                                href: "{href}",
                                target: link.target,
                                rel: link.rel,
                                download: link.download,
                                "{label}"
                            }
                        }
                    })}
                }
            }
            // Sits after the body so a phone reads the name first. CSS lifts
            // it back into the margin under the portrait on desktop.
            div {
                class: "hero-docket reveal",
                span { "Sofia \u{00B7} Bulgaria" }
                span { "{years}+ years\u{2019} experience" }
                span { "PhD \u{00B7} MEng" }
            }
        }

        // `.image-overlay[open]` in index.css controls visibility. Escape is
        // the browser's to handle, so there is no key handler to miss.
        dialog {
            id: "portrait-dialog",
            class: "image-overlay",
            aria_label: "Portrait of Alexander Alexandrov",
            onclick: move |_| close_portrait(),
            button {
                r#type: "button",
                class: "close-button",
                aria_label: "Close portrait",
                onclick: move |e: MouseEvent| {
                    e.stop_propagation();
                    close_portrait();
                },
                "\u{00D7}"
            }
            // A second tab stop; with only one, Tab escapes to <body>.
            img {
                src: asset!("/assets/images/profilepic.jpeg"),
                alt: "Alexander Alexandrov",
                width: "640",
                height: "640",
                tabindex: "0",
                onclick: move |e: MouseEvent| {
                    e.stop_propagation();
                }
            }
        }
    }
}
