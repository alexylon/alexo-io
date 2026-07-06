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
            h2 { "Skills" }
            div {
                class: "skills-rows",
                {SKILL_CATEGORIES.iter().map(|cat| rsx! {
                    div {
                        class: "skill-row",
                        span { class: "apparatus", "{cat.name}" }
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
