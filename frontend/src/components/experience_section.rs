use crate::components::data::EXPERIENCE_ENTRIES;
use crate::components::entry_card::EntryCard;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn ExperienceSection(experience_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            id: "experience",
            onmounted: move |cx| experience_section.set(Some(cx.data())),
            class: "experience-section section",
            h2 { "Experience" }
            div {
                class: "entry-list",
                {EXPERIENCE_ENTRIES.iter().map(|entry| rsx! {
                    EntryCard {
                        apparatus: rsx! { "{entry.period}" },
                        title: rsx! { "{entry.title}" },
                        org: rsx! { "{entry.company}" },
                        items: entry.responsibilities.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                    }
                })}
            }
        }
    }
}
