use dioxus::prelude::*;
use dioxus_desktop::use_window;
use crate::{
    app::titlebar::control_box::ControlBox, localization::{use_translator, Language}, merge_styles
};

#[component]
pub fn TitleBar(title: String) -> Element {
    let window = use_window();
    let t = use_translator();
    let mut is_maximized = use_signal(|| window.is_maximized());


    let ar_padding = use_signal(|| Some("padding-right: 1rem; padding-left: 0.3rem;"));
    let en_padding = use_signal(|| Some("padding-right: 0.4rem; padding-left: 1rem;"));
    let padding = if t.get_language() == Language::Arabic { ar_padding } else { en_padding };
    
    rsx! {
        div {
            // Drag region and styling
            class: "flex items-center justify-between h-10 min-h-10 bg-background",
            style: merge_styles!(Some("-webkit-app-region: drag;"), padding()),
            direction: if t.get_language() == Language::Arabic {"rtl"} else {"ltr"},
            onresize: move |_| {
                is_maximized.set(window.is_maximized());
            },
            "data-tauri-drag-region": "true",

            // Title element
            div {
                class: "text-sm",
                "data-tauri-drag-region": "",
                "{title}"
            }

            // Control box
            ControlBox {
                is_maximized: is_maximized,
            }
        }
    }
}
