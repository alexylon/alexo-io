use crate::components::data::SKILL_CATEGORIES;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn SkillsSection(skills_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            id: "skills",
            onmounted: move |cx| skills_section.set(Some(cx.data())),
            class: "skills-section section",
            h2 { "Technical Skills" }
            div {
                class: "skills-categories",
                {SKILL_CATEGORIES.iter().map(|cat| rsx! {
                    div {
                        class: "skill-group",
                        span { class: "skill-group-label", "{cat.name}" }
                        div {
                            class: "skills-grid",
                            {cat.skills.iter().map(|skill| rsx! {
                                span {
                                    class: "chip",
                                    "{skill}"
                                }
                            })}
                        }
                    }
                })}
            }
        }
    }
}
