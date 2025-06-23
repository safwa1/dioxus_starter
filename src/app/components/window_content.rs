use dioxus::prelude::*;
use dioxus_desktop::use_window;

use crate::localization::{use_translator, Language};


#[component]
pub fn WindowContent(content: Element) -> Element {
    let t = use_translator();
    let window = use_window();
    rsx! {
        div {
            class: "window-container themeable",
            style: if window.is_decorated() {"--height-offset:  20px; --mt: 10px; --mb: 10px; --mr: 10px; --ml: 10px"} else {"--height-offset: 55px"},
            direction: if t.get_language() == Language::Arabic {"rtl"} else {"ltr"},
            {content}
        }
    }
}