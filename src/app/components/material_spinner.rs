use dioxus::prelude::*;

use crate::{cn, merge_styles};

#[derive(Props, PartialEq, Clone)]
pub struct SpinnerProps {
    #[props(optional)]
    class: Option<String>,
    
    #[props(optional)]
    style: Option<String>,

    #[props(default = "18px".to_string())]
    width: String,

    #[props(default = "18px".to_string())]
    height: String,

    #[props(default = "10px".to_string())]
    margin_left: String,
    
}

#[component]
pub fn MaterialSpinner(props: SpinnerProps) -> Element {
    let base_styles = format!( "width:{}; height:{}; margin-left:{}; flex-shrink:0;", props.width, props.height, props.margin_left);
    rsx! {
        document::Stylesheet { href: asset!("/assets/style/components/material-spinner.css")}
        svg {
            class: cn!("spinner", props.class.as_deref().unwrap_or("")),
            style: merge_styles!(Some(base_styles), props.style),
            view_box: "0 0 66 66",
            xmlns: "http://www.w3.org/2000/svg",

            circle {
                class: "path",
                fill: "none",
                stroke_width: "6",
                stroke_linecap: "round",
                cx: "33",
                cy:"33",
                r:"30"
            }
        }
    }
}