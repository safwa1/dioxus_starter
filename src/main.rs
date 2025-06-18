#![windows_subsystem = "windows"]
#![allow(dependency_on_unit_never_type_fallback)]

use dioxus::prelude::*;
use dioxus_desktop::tao::platform::windows::WindowBuilderExtWindows;
#[cfg(feature = "desktop")]
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_starter::{app::App};

const WINDOW_WIDTH: u32 = 820;
const WINDOW_HEIGHT: u32 = 850;


fn main() {
    let window = WindowBuilder::new()
        .with_title("Dioxus Starter")
        .with_inner_size(dioxus_desktop::LogicalSize::new(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ))
        .with_minimizable(true)
        .with_maximizable(true)
        .with_decorations(false)
        .with_undecorated_shadow(true)
        .with_transparent(true)
        .with_resizable(true);

    #[cfg(feature = "desktop")]
    let config = Config::new()
        .with_menu(None)
        .with_disable_context_menu(!cfg!(debug_assertions))
        .with_window(window);

    LaunchBuilder::new().with_cfg(config).launch(App);
}
