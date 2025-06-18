use dioxus::prelude::*;


#[component]
pub fn Checkbox(checked: Signal<bool>, on_change: EventHandler<bool>) -> Element {
    rsx! {
        div { class: "inline-flex items-center",
            label { class: "flex items-center cursor-pointer relative",
                input {
                    class: "peer h-5 w-5 cursor-pointer transition-all appearance-none rounded shadow hover:shadow-md border border-slate-300 checked:bg-slate-800 checked:border-slate-800",
                    r#type: "checkbox",
                    checked: "{checked}",
                    onchange: move |e| on_change.call(e.checked()),
                    id: "check"
                }
                span { 
                    class: "absolute text-white opacity-0 peer-checked:opacity-100 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 pointer-events-none",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-3.5 w-3.5",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        stroke: "currentColor",
                        stroke_width: "1",
                        path {
                            fill_rule: "evenodd",
                            d: "M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z",
                            clip_rule: "evenodd"
                        }
                    }
                }
            }
        }
    }
}