use crate::components::ScrollCleanup;
use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn ScrollToTop(
    top_element: Signal<Option<Rc<MountedData>>>,
    theme: Signal<crate::Theme>,
) -> Element {
    let mut show_button = use_signal(|| false);

    // Register scroll listener once; ScrollCleanup removes it on drop
    let _cleanup: Option<Rc<ScrollCleanup>> = use_hook(|| {
        let window = web_sys::window()?;

        let handle_scroll = {
            let mut show_button = show_button.clone();
            move || {
                let scroll_y = web_sys::window()
                    .and_then(|w| w.page_y_offset().ok())
                    .unwrap_or(0.0);
                show_button.set(scroll_y > 150.0);
            }
        };

        let closure = Closure::new(handle_scroll);
        window
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .ok()?;

        let initial_scroll = window.page_y_offset().unwrap_or(0.0);
        show_button.set(initial_scroll > 150.0);

        Some(Rc::new(ScrollCleanup { closure }))
    });

    rsx! {
        button {
            r#type: "button",
            class: "scroll-to-top",
            class: if show_button() { "" } else { "hidden" },
            aria_label: "Scroll to top",
            onclick: move |_| async move {
                if let Some(header) = top_element.cloned() {
                    header.scroll_to(crate::preferred_scroll_behavior()).await.ok();
                }
            },
            img {
                src: "{theme().icon_up()}",
                alt: "",
            }
        }
    }
}
