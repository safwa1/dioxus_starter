use crate::app::material_spinner::MaterialSpinner;
use crate::app::titlebar::title_bar::TitleBar;
use crate::app::window_content::WindowContent;
use crate::app::Route;
use crate::localization::use_translator;
use crate::state::app_state::AppState;
use dioxus::prelude::*;
use dioxus_desktop::tao::platform::windows::WindowExtWindows;
use dioxus_desktop::wry::dpi::PhysicalPosition;
use dioxus_desktop::{use_window, DesktopContext, DesktopService};
use tokio::time::sleep;
use std::rc::Rc;

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

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(AppState::default()));

    let desktop: Rc<DesktopService> = use_window();

    desktop.set_undecorated_shadow(true);
    let t = use_translator();
    
    // State to manage loading
    let mut is_loading = use_signal(|| true); 

    use_effect(move || {
        
        spawn(async move {            
            sleep(std::time::Duration::from_secs(1)).await; 
            is_loading.set(false); // Set loading to false once everything is loaded
        });
    
        center_window(&desktop);
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css")}
        document::Stylesheet { href: asset!("/assets/style/app.css")}
        
        TitleBar { title: t.text_or("app_title", "Dioxus Starter Kit") }
        
        if is_loading() {
            Loader { }
        } else {
            WindowContent {
                content: rsx! { Router::<Route> {} }
            }
        }
    }
}

#[component]
fn Loader() -> Element {
    rsx! {
        div {
            style:r#"
                background-color: var(--window-background);
                margin: 0.7rem;
                margin-right: 0.76rem;
                margin-top: 2px;
                box-sizing: border-box;
                border: 1px solid var(--window-border);
                border-radius: 7px;
                height: calc(100vh - 55px);
                padding: 20px;
                display: flex;
                flex-direction: column;
                overflow-y: auto;
            "#,
            class: "items-center justify-center",
            MaterialSpinner { width: "50px", height: "50px"}
        }
    }
}