use dioxus::prelude::*;

use crate::{cn, state::app_state::use_app_state};

#[derive(Props, PartialEq, Clone)]
pub struct WindowControlButtonProps {
    #[props(optional)]
    icon: Option<Element>,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    on_click: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn ControlButton(props: WindowControlButtonProps) -> Element {
    let state = use_app_state();
    let base_classes = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none h-8 w-8 p-0 cursor-pointer";
    rsx! {
        button {
            class: cn!(base_classes, if state().is_dark() { "hover:bg-[#1C1C20]" } else { "hover:bg-[#EAEBEF]" },  props.class.as_deref().unwrap_or("")),
            onclick: move |evt| {
                if let Some(handler) = props.on_click.as_ref() {
                    handler.call(evt);
                }
            },

            {props.icon}
        }
    }
}