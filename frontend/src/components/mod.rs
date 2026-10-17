pub mod certifications_section;
pub mod contact_section;
mod data;
pub mod education_section;
mod email_link;
pub mod entry_card;
pub mod experience_section;
pub mod footer_section;
pub mod hero_section;
pub mod languages_section;
pub mod nav_section;
pub mod projects_section;
pub mod scroll_to_top;
pub mod skills_section;

// Removes the window "scroll" listener when the owning component unmounts.
#[cfg(target_arch = "wasm32")]
pub(crate) struct ScrollCleanup {
    pub closure: wasm_bindgen::closure::Closure<dyn FnMut()>,
}

#[cfg(target_arch = "wasm32")]
impl Drop for ScrollCleanup {
    fn drop(&mut self) {
        use wasm_bindgen::JsCast;
        if let Some(window) = web_sys::window() {
            let _ = window.remove_event_listener_with_callback(
                "scroll",
                self.closure.as_ref().unchecked_ref(),
            );
        }
    }
}

// In-page moves that never push a history entry, so Back leaves the site
// instead of retracing the visitor's own clicks. No `behavior` argument, so
// `html { scroll-behavior }` still governs reduced motion.

#[cfg(target_arch = "wasm32")]
pub(crate) fn go_to_section(id: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    if let Some(el) = window.document().and_then(|d| d.get_element_by_id(id)) {
        // scroll-margin-top on the section clears the fixed bar.
        el.scroll_into_view();
        take_focus(&el);
    }
    replace_url(&window, &format!("#{id}"));
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn go_to_top() {
    let Some(window) = web_sys::window() else {
        return;
    };
    window.scroll_to_with_x_and_y(0.0, 0.0);
    if let Some(el) = window.document().and_then(|d| d.get_element_by_id("top")) {
        take_focus(&el);
    }
    // The top is the bare page, so drop any section hash. Whatever referral
    // query the visitor arrived with is theirs to keep.
    let location = window.location();
    let mut url = location.pathname().unwrap_or_default();
    url.push_str(&location.search().unwrap_or_default());
    replace_url(&window, &url);
}

/// Gives back what a plain `#id` navigation does and `prevent_default` takes
/// away: the next Tab carries on from the destination instead of the nav. The
/// targets carry `tabindex="-1"`, so this reaches them without adding tab
/// stops. `prevent_scroll` keeps focus from cancelling the smooth scroll.
#[cfg(target_arch = "wasm32")]
fn take_focus(el: &web_sys::Element) {
    use wasm_bindgen::JsCast;
    if let Some(el) = el.dyn_ref::<web_sys::HtmlElement>() {
        let options = web_sys::FocusOptions::new();
        options.set_prevent_scroll(true);
        let _ = el.focus_with_options(&options);
    }
}

#[cfg(target_arch = "wasm32")]
fn replace_url(window: &web_sys::Window, url: &str) {
    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(url));
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn go_to_section(_id: &str) {}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn go_to_top() {}

pub use certifications_section::CertificationsSection;
pub use contact_section::ContactSection;
pub use education_section::EducationSection;
pub use experience_section::ExperienceSection;
pub use footer_section::FooterSection;
pub use hero_section::HeroSection;
pub use languages_section::LanguagesSection;
pub use nav_section::NavSection;
pub use projects_section::ProjectsSection;
pub use scroll_to_top::ScrollToTop;
pub use skills_section::SkillsSection;
