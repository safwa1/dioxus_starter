use dioxus::prelude::*;
use dioxus_desktop::use_window;
use crate::{
    app::titlebar::control_box::ControlBox, localization::{Language}, merge_styles, state::app_state::use_app_state
};

#[component]
pub fn TitleBar(title: String) -> Element {
    let window = use_window();
    let app_state  = use_app_state();
    let mut is_maximized = use_signal(|| window.is_maximized());

    // Derive is_arabic directly from app_state, no need for a separate signal
    let is_arabic = move || app_state().language() == Language::Arabic;

    // Use a computed signal for padding based on is_arabic
    let padding = use_memo(move || {
        if is_arabic() {
            "padding-right: 1rem; padding-left: 0.3rem;"
        } else {
            "padding-right: 0.4rem; padding-left: 1rem;"
        }
    });

    // Use a computed signal for direction based on is_arabic
    let direction = use_memo(move || {
        if is_arabic() {
            "rtl"
        } else {
            "ltr"
        }
    });
    
    rsx! {
        div {
            // Drag region and styling
            class: "flex items-center justify-between h-10 min-h-10 bg-background",
            style: merge_styles!(Some("-webkit-app-region: drag;"), Some(padding())),
            direction: "{direction()}",
            onresize: move |_| {
                is_maximized.set(window.is_maximized());
            },

            // Title element
            div {
                class: "text-sm",
                "{title}"
            }

            // Control box
            ControlBox {
                is_maximized: is_maximized,
            }
        }
    }
}
