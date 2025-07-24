#![windows_subsystem = "windows"]
#![allow(dependency_on_unit_never_type_fallback)]

use dioxus::prelude::*;
use dioxus_desktop::tao::{event_loop::EventLoop, platform::windows::WindowBuilderExtWindows};
#[cfg(feature = "desktop")]
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_starter::{app::App, utils::appname_utils::AppName};

const DEFAULT_WINDOW_WIDTH: u32 = 620;
const DEFAULT_WINDOW_HEIGHT: u32 = 650;
const GOLDEN_RATIO: f64 = 1.6180339887;

fn main() {
    let (window_width, window_height) = calculate_initial_window_size();
    println!("width = {}, height = {}", window_width, window_height);

    let window = WindowBuilder::new()
        .with_title(AppName::get_formatted_value())
        .with_inner_size(dioxus_desktop::LogicalSize::new(
            window_width,
            window_height,
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

fn calculate_initial_window_size() -> (u32, u32) {
    // Create an EventLoop to access monitor information
    let event_loop = EventLoop::new();

    // Get the primary monitor or the first available monitor
    if let Some(monitor) = event_loop
        .primary_monitor()
        .or_else(|| event_loop.available_monitors().next())
    {
        let monitor_size = monitor.size();
        (
            (monitor_size.width / 3),
            ((monitor_size.height / 4) as f64 * GOLDEN_RATIO) as u32,
        )
    } else {
        println!(
            "Could not determine monitor size, using default width = {}, height = {}",
            DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT
        );
        (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT) // return default size
    }
}
