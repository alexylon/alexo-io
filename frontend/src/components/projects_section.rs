use crate::components::data::PROJECTS;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn ProjectsSection(projects_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            id: "projects",
            onmounted: move |cx| projects_section.set(Some(cx.data())),
            class: "projects-section section",
            h2 { "Open-Source Projects" }
            div {
                class: "works-list",
                {PROJECTS.iter().map(|project| rsx! {
                    article {
                        class: "work",
                        div {
                            class: "apparatus",
                            "{project.kind}"
                        }
                        h3 {
                            class: "work-name",
                            a {
                                href: "{project.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "{project.name}"
                            }
                        }
                        p {
                            class: "work-desc",
                            "{project.description}"
                        }
                        div {
                            class: "work-links",
                            a {
                                class: "work-link",
                                href: "{project.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "Source code"
                            }
                            if let Some(homepage) = project.homepage {
                                {
                                    let display = homepage.trim_start_matches("https://");
                                    rsx! {
                                        a {
                                            class: "work-link",
                                            href: "{homepage}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            "{display}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
}
