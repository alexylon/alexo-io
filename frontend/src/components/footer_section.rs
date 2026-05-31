use dioxus::prelude::*;

#[component]
pub fn FooterSection() -> Element {
    let year = js_sys::Date::new_0().get_full_year();

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
