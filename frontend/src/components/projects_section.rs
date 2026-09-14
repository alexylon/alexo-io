use crate::components::data::PROJECTS;
use dioxus::prelude::*;

#[component]
pub fn ProjectsSection() -> Element {
    rsx! {
        section {
            id: "projects",
            class: "projects-section section",
            tabindex: "-1",
            // Matches the nav label. The rail kinds and the GitHub links
            // already say these are open source.
            h2 { "Projects" }
            div {
                class: "works-list",
                {PROJECTS.iter().map(|project| {
                    // The name goes where the project lives: its own site when
                    // it has one, otherwise the repository. The row beneath
                    // still names every destination.
                    let primary = project.homepage.unwrap_or(project.url);
                    rsx! {
                        article {
                            class: "work",
                            div {
                                class: "apparatus",
                                "{project.kind}"
                            }
                            h3 {
                                class: "work-name",
                                a {
                                    href: "{primary}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "{project.name}"
                                }
                            }
                            p {
                                class: "work-desc",
                                "{project.description}"
                            }
                            // Each label is the place it lands on, so the row
                            // reads in one register: GitHub, crates.io, the
                            // project's own domain.
                            div {
                                class: "work-links",
                                a {
                                    class: "work-link",
                                    href: "{project.url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "GitHub"
                                }
                                if let Some(crate_url) = project.crate_url {
                                    a {
                                        class: "work-link",
                                        href: "{crate_url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "crates.io"
                                    }
                                }
                                if let Some(homepage) = project.homepage {
                                    a {
                                        class: "work-link",
                                        href: "{homepage}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        {display_domain(homepage)}
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

/// `https://www.clavir.io` reads as `clavir.io`: the scheme and `www.` are the
/// address's business, not the visitor's.
fn display_domain(url: &str) -> &str {
    url.trim_start_matches("https://")
        .trim_start_matches("www.")
        .trim_end_matches('/')
}
