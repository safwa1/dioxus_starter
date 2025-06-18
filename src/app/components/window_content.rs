use dioxus::prelude::*;

use crate::localization::{use_translator, Language};


#[component]
pub fn WindowContent(content: Element) -> Element {
    let t = use_translator();
    rsx! {
        document::Stylesheet { href: asset!("/assets/style/components/window.css")}
        div {
            class: "window-container themeable",
            direction: if t.get_language() == Language::Arabic {"rtl"} else {"ltr"},
            {content}
        }
    }
}