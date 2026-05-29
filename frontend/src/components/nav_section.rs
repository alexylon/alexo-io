use crate::components::ScrollCleanup;
use crate::Theme;
use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

const SECTION_IDS: &[&str] = &["skills", "experience", "projects", "education", "contact"];

#[component]
fn NavLink(
    label: &'static str,
    section: Signal<Option<Rc<MountedData>>>,
    is_active: bool,
    tabbable: bool,
) -> Element {
    let class = if is_active {
        "nav-link active"
    } else {
        "nav-link"
    };
    rsx! {
        button {
            r#type: "button",
            class: "{class}",
            tabindex: if tabbable { "0" } else { "-1" },
            onclick: move |_| async move {
                if let Some(el) = section.cloned() {
                    el.scroll_to(crate::preferred_scroll_behavior()).await.ok();
                }
            },
            "{label}"
        }
    }
}

fn focus_nav_sibling(forward: bool) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(active) = document.active_element() else {
        return;
    };
    let Ok(buttons) = document.query_selector_all(".nav-bar button") else {
        return;
    };

    let len = buttons.length();
    let mut current = 0;
    for i in 0..len {
        if let Some(node) = buttons.item(i) {
            if node == *active {
                current = i;
                break;
            }
        }
    }

    let next = if forward {
        if current + 1 < len {
            current + 1
        } else {
            0
        }
    } else {
        if current > 0 {
            current - 1
        } else {
            len - 1
        }
    };

    if let Some(node) = buttons.item(next) {
        if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}

#[component]
pub fn NavSection(
    theme: Signal<Theme>,
    active_section: Signal<String>,
    skills_section: Signal<Option<Rc<MountedData>>>,
    experience_section: Signal<Option<Rc<MountedData>>>,
    projects_section: Signal<Option<Rc<MountedData>>>,
    education_section: Signal<Option<Rc<MountedData>>>,
    contact_section: Signal<Option<Rc<MountedData>>>,
) -> Element {
    let _cleanup: Option<Rc<ScrollCleanup>> = use_hook(|| {
        let window = web_sys::window()?;
        let mut active = active_section.clone();
        let prev_scroll = std::cell::Cell::new(0.0_f64);

        let closure = Closure::<dyn FnMut()>::new(move || {
            let Some(window) = web_sys::window() else {
                return;
            };
            let Some(document) = window.document() else {
                return;
            };
            let Some(doc_el) = document.document_element() else {
                return;
            };

            let viewport_h = window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let scroll_y = window.page_y_offset().unwrap_or(0.0);
            let doc_height = doc_el.scroll_height() as f64;
            let scrolling_down = scroll_y > prev_scroll.get();
            prev_scroll.set(scroll_y);

            if scroll_y + viewport_h >= doc_height - 50.0 {
                active.set("contact".to_string());
                return;
            }

            // Scrolling down: activate when heading reaches the top (10%)
            // Scrolling up: keep section active while heading is visible (30%)
            let threshold = if scrolling_down {
                viewport_h * 0.15
            } else {
                viewport_h * 0.3
            };

            let mut active_id = String::new();
            for id in SECTION_IDS {
                if let Some(el) = document.get_element_by_id(id) {
                    if el.get_bounding_client_rect().top() < threshold {
                        active_id = id.to_string();
                    }
                }
            }

            active.set(active_id);
        });

        window
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .ok()?;

        Some(Rc::new(ScrollCleanup { closure }))
    });

    let active = active_section();
    let has_active = !active.is_empty();
    let theme_label = match theme() {
        Theme::Dark => "Switch to light theme",
        Theme::Light => "Switch to dark theme",
    };
    let is_dark = matches!(theme(), Theme::Dark);

    rsx! {
        nav {
            class: "fixed-nav",
            div {
                class: "nav-bar",
                role: "toolbar",
                aria_label: "Section navigation",
                onkeydown: move |e: KeyboardEvent| {
                    match e.key() {
                        Key::ArrowRight => {
                            e.prevent_default();
                            focus_nav_sibling(true);
                        }
                        Key::ArrowLeft => {
                            e.prevent_default();
                            focus_nav_sibling(false);
                        }
                        Key::Escape => {
                            if let Some(window) = web_sys::window() {
                                if let Some(doc) = window.document() {
                                    if let Some(active) = doc.active_element() {
                                        if let Ok(el) = active.dyn_into::<web_sys::HtmlElement>() {
                                            let _ = el.blur();
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                },
                div {
                    class: "nav-links",
                    NavLink { label: "Skills", section: skills_section, is_active: active == "skills", tabbable: active == "skills" || !has_active }
                    NavLink { label: "Experience", section: experience_section, is_active: active == "experience", tabbable: active == "experience" }
                    NavLink { label: "Projects", section: projects_section, is_active: active == "projects", tabbable: active == "projects" }
                    NavLink { label: "Education", section: education_section, is_active: active == "education", tabbable: active == "education" }
                    NavLink { label: "Contact", section: contact_section, is_active: active == "contact", tabbable: active == "contact" }
                }
                button {
                    r#type: "button",
                    class: "theme-toggle",
                    tabindex: "-1",
                    aria_label: "{theme_label}",
                    aria_pressed: "{is_dark}",
                    onclick: move |_| {
                        let new_theme = theme().toggle();
                        theme.set(new_theme);
                        spawn(async move {
                            crate::theme_store::save_theme(new_theme).await;
                        });
                    },
                    img {
                        src: "{theme().icon_theme()}",
                        alt: "",
                        width: "22",
                    }
                }
            }
        }
    }
}
