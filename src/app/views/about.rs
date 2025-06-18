use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use crate::{app::Route, localization::use_translator};

#[component]
pub fn About(name: String) -> Element {
    let t = use_translator();
    
    let mut show_content = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            show_content.set(true);
        });
    });
    
    let transition_classes = if *show_content.read() {
        // Final state: fully opaque, no translation
        "opacity-100 translate-y-0"
    } else {
        // Initial state: fully transparent, slightly moved down (or up, depending on desired effect)
        "opacity-0 translate-y-4" // Fades in from slightly below
        // Or "opacity-0 -translate-y-4" for fading in from slightly above
    };

    rsx! {
        div {
            // Apply transition properties to the container
            class: "flex flex-col items-center justify-center h-full transition-all duration-700 ease-out {transition_classes}",
            
            h1 {
                class: "mb-2",
                {t.text("about_page_label")}
            }

            p {
                class: "mb-2",
                {t.formatted_text("welcome_message", &[&name])}
            }
            
            Link {
                to: Route::Home { },
                class: "text-blue-500 hover:underline cursor-pointer font-sans text-base",
                {t.text("back_to_home")}
            }
        }
    }
}