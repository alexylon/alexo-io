use dioxus::prelude::*;

#[component]
pub fn FooterSection() -> Element {
    let year = crate::use_current(crate::current_year);

    rsx! {
        // Contact closes the page. This is only the notice line.
        footer {
            class: "footer-section",
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
