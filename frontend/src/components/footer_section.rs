use dioxus::prelude::*;

#[component]
pub fn FooterSection() -> Element {
    let year = crate::current_year();

    rsx! {
        footer {
            class: "footer-section",
            p {
                class: "footer-badge",
                "Built with Rust"
            }
            p { "\u{00A9} {year} Alexander Alexandrov" }
            p {
                a {
                    href: "https://github.com/alexylon/alexo-io",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Source code"
                }
            }
        }
    }
}
