pub mod about_section;
pub mod certifications_section;
pub mod contact_section;
mod data;
pub mod education_section;
pub mod experience_section;
pub mod footer_section;
pub mod header_section;
pub mod languages_section;
pub mod nav_section;
pub mod projects_section;
pub mod scroll_to_top;
pub mod skills_section;
pub mod timeline_card;

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

pub use about_section::AboutSection;
pub use certifications_section::CertificationsSection;
pub use contact_section::ContactSection;
pub use education_section::EducationSection;
pub use experience_section::ExperienceSection;
pub use footer_section::FooterSection;
pub use header_section::HeaderSection;
pub use languages_section::LanguagesSection;
pub use nav_section::NavSection;
pub use projects_section::ProjectsSection;
pub use scroll_to_top::ScrollToTop;
pub use skills_section::SkillsSection;
