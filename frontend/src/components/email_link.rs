use crate::components::data::EMAIL;
use dioxus::prelude::*;

// Runs before first paint. Only rewrites the text node: Dioxus needs the
// hydration comments around it.
const DECODE_SCRIPT: &str = "(function(){\
var a=document.currentScript.previousElementSibling,h=a&&a.getAttribute('data-email');\
if(!h)return;\
var k=parseInt(h.slice(0,2),16),s='';\
for(var i=2;i<h.length;i+=2)s+=String.fromCharCode(parseInt(h.slice(i,i+2),16)^k);\
a.href='mailto:'+s;\
for(var n=a.firstChild;n;n=n.nextSibling)if(n.nodeType===3)n.data=s;\
})();";

#[component]
pub fn EmailLink(class: &'static str) -> Element {
    // Starts false so the first client render matches the prerendered HTML.
    let mut decoded = use_signal(|| false);
    use_effect(move || decoded.set(true));

    let address = decoded().then(|| EMAIL.address());
    let href = address.as_ref().map(|address| format!("mailto:{address}"));
    let text = address.unwrap_or_else(|| EMAIL.label.to_string());

    rsx! {
        a {
            class: "{class}",
            href: href,
            "data-email": EMAIL.hex(),
            "{text}"
        }
        script { dangerous_inner_html: DECODE_SCRIPT }
    }
}
