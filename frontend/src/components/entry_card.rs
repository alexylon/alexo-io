use dioxus::prelude::*;

/// One entry with an optional margin-rail `apparatus` (dates, kinds), a title,
/// an optional `org`, and bullet points. Used by experience, education, and
/// certifications.
#[component]
pub fn EntryCard(
    #[props(default)] apparatus: Option<Element>,
    title: Element,
    #[props(default)] org: Option<Element>,
    #[props(default)] items: Vec<String>,
) -> Element {
    rsx! {
        div {
            class: "entry",
            if let Some(apparatus) = apparatus {
                div {
                    class: "apparatus",
                    {apparatus}
                }
            }
            h3 {
                class: "entry-title",
                {title}
            }
            if let Some(org) = org {
                p {
                    class: "entry-org",
                    {org}
                }
            }
            if !items.is_empty() {
                ul {
                    {items.iter().map(|item| rsx! {
                        li { "{item}" }
                    })}
                }
            }
        }
    }
}
