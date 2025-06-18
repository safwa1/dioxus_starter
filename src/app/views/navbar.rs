use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}
