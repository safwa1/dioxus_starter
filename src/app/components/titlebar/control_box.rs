use dioxus::prelude::*;
use dioxus_desktop::use_window;

use crate::app::titlebar::{
    control_button::ControlButton, 
    icons::icons::{ 
        CloseWin,
        MaximizeWin, 
        MinimizeWin, 
        RestoreWin
    }
};

#[component]
pub fn ControlBox(is_maximized: Signal<bool>) -> Element {
    let window = use_window();

    let ic = if is_maximized() {
        Some(rsx! { RestoreWin { class: "width: 10px; height:11px" } })
    } else {
        Some(rsx! { MaximizeWin { class: "width: 10px; height: 10px" } })
    };

    rsx! {
        div { class: "flex items-center gap-1",
            
            if window.is_minimizable() {
                // Minimize Button
                ControlButton {
                    icon: rsx! { MinimizeWin { class: "width: 10px; height: 10px" } },
                    on_click: {
                        let window = window.clone();
                        move |_| window.set_minimized(true)
                    },
                }
            }


            if window.is_maximizable() {
                // Maximize/Restore Button
                ControlButton {
                    icon: ic,
                    on_click: {
                        let window = window.clone();
                        move |_| {
                            window.set_maximized(!window.is_maximized());
                            is_maximized.set(window.is_maximized());
                        }
                    },
                }
            }
            

            // Close Button
            ControlButton {
                icon: rsx! { CloseWin { class: "width: 10px; height: 10px"  } },
                on_click: {
                    let window = window.clone();
                    move |_| window.close()
                },
            }
        }
    }
}
