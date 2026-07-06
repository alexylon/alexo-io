use dioxus::prelude::*;

#[component]
pub fn FooterSection() -> Element {
    let year = crate::current_year();

    rsx! {
        footer {
            class: "footer-section",
            span {
                class: "colophon-label",
                "Colophon"
            }
            p {
                "Built with Rust \u{2014} Dioxus on the client, axum on the server \u{2014} \
                and served from a Raspberry Pi in Sofia."
            }
            p {
                class: "footer-meta",
                "\u{00A9} {year} Alexander Alexandrov \u{00B7} "
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
