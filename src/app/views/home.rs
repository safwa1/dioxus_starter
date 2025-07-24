use dioxus::prelude::*;
use dioxus_desktop::use_window;
use crate::{
    app::Route,
    localization::use_translator,
    state::app_state::use_app_state,
};

#[component]
pub fn Home() -> Element {
    let t = use_translator();
    let mut app_state = use_app_state();
    let mut show_content = use_signal(|| false);
    let mut change_decoration_text = use_signal(|| "Restore Default Window Decoration");
    let window = use_window();

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
            class: "flex flex-col items-center justify-center h-full transition-all duration-700 ease-out {transition_classes}",

            h1 {
                {t.text("home_page_label")}
            }

            Link {
                to: Route::About { name: "Safwan".to_string() },
                class: "text-blue-500 hover:underline mt-2 cursor-pointer font-sans",
                {t.text("go_to_about")}
            }

            button {
                class: "button themeable",
                onclick: move |_| {
                    app_state.write().toggle_language();
                },
                {t.text("switch_language")}
             }

            button {
                class: "button themeable",
                onclick: move |_| {
                    app_state.write().toggle_theme();
                },
                {t.text("switch_theme")}
            }

            button {
                class: "button themeable",
                onclick: move |_| {
                    window.set_decorations(!window.is_decorated());
                    change_decoration_text.set(if window.is_decorated() { "Use Custom Window Decoration" } else { "Restore Default Window Decoration" });
                },
                "{change_decoration_text}"
            }

         }

    }
}
