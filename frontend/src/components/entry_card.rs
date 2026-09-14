use dioxus::prelude::*;

/// One entry: an optional rail label, a title, an optional organisation, and
/// bullet points. Used by experience, education and certifications.
#[component]
pub fn EntryCard(
    #[props(default)] apparatus: Option<Element>,
    title: Element,
    #[props(default)] org: Option<Element>,
    #[props(default)] items: Vec<String>,
    /// Outbound links below the entry, styled like the projects list, so a
    /// title never has to double as a link.
    #[props(default)]
    links: Option<Element>,
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
            if let Some(links) = links {
                div {
                    class: "work-links",
                    {links}
                }
            }
        }
    }
}
