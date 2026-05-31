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
                class: "projects-grid",
                {PROJECTS.iter().map(|project| rsx! {
                    article {
                        class: "project-card",
                        div {
                            class: "project-card-header",
                            h3 {
                                class: "project-card-name",
                                a {
                                    class: "project-card-title-link",
                                    href: "{project.url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "{project.name}"
                                }
                            }
                        }
                        p {
                            class: "project-card-desc",
                            "{project.description}"
                        }
                        div {
                            class: "project-card-links",
                            a {
                                class: "project-card-link",
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
                                            class: "project-card-link",
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
