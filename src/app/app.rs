use crate::app::material_spinner::MaterialSpinner;
use crate::app::titlebar::title_bar::TitleBar;
use crate::app::window_content::WindowContent;
use crate::app::Route;
use crate::localization::use_translator;
use crate::state::app_state::{use_app_state, AppState};
use crate::utils::appname_utils::AppName;
use dioxus::prelude::*;
use dioxus_desktop::wry::dpi::PhysicalPosition;
use dioxus_desktop::{use_window, DesktopContext, DesktopService};
use std::rc::Rc;
use tokio::time::sleep;

#[component]
pub fn App() -> Element {
    // Load the initial AppState
    let initial_app_state = AppState::load();
    use_context_provider(|| Signal::new(initial_app_state.clone()));
    let app_state = use_app_state();
    let desktop: Rc<DesktopService> = use_window();
    let t = use_translator();

    let decorated_state = desktop.is_decorated();

    let mut is_loading = use_signal(|| true);

    use_effect(move || {
        let current_theme = app_state().theme();
        app_state().apply_theme(current_theme);

        if is_loading() {
            center_window(&desktop);
        }

        spawn(async move {
            sleep(std::time::Duration::from_secs(1)).await;
            is_loading.set(false);
        });
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/style/app.css")}
        document::Stylesheet { href: asset!("/assets/style/components/window.css")}
        document::Stylesheet { href: asset!("/assets/tailwind.css")}

        // use custom title-bar only when window not use default decoration
        if !decorated_state {
            TitleBar { title: t.text_or("app_title", AppName::get_formatted_value()) }
        }

        if is_loading() {
            Loader { decorated_state: decorated_state }
        } else {
            WindowContent {
                content: rsx! { Router::<Route> {} }
            }
        }
    }
}

#[component]
pub fn Loader(decorated_state: bool) -> Element {
    let container_style = use_memo(move || {
        if decorated_state {
            "--height-offset: 20px; --mt: 10px; --mb: 10px; --mr: 10px; --ml: 10px"
        } else {
            "--height-offset: 55px"
        }
    });

    rsx! {
        div {
            class: "window-container themeable items-center justify-center",
            style: "{container_style}",
            MaterialSpinner { width: "50px", height: "50px"}
        }
    }
}

fn center_window(desktop_context: &DesktopContext) {
    if let Some(monitor) = desktop_context.current_monitor() {
        let monitor_size = monitor.size();
        let window = &desktop_context.window;
        let window_size = window.outer_size();

        let x = (monitor_size.width as i32 - window_size.width as i32) / 2;
        let y = (monitor_size.height as i32 - window_size.height as i32) / 2;

        // FIX: Use PhysicalPosition to match the calculation units.
        window.set_outer_position(PhysicalPosition::new(x.max(0), y.max(0)));
    }
}
