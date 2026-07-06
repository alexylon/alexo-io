use crate::components::data::EDUCATION;
use crate::components::entry_card::EntryCard;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn EducationSection(education_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            id: "education",
            onmounted: move |cx| education_section.set(Some(cx.data())),
            class: "education-section section",
            h2 { "Education" }
            div {
                class: "entry-list entry-list-tight",
                {EDUCATION.iter().map(|ed| rsx! {
                    EntryCard {
                        title: rsx! { "{ed.title}" },
                        org: rsx! { "{ed.institution}" },
                    }
                })}
            }
        }
    }
}
