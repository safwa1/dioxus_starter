use dioxus::prelude::*;

use crate::app::{
    views::Home,
    views::About,
    navbar::Navbar
};


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},

        #[route("/about/:name")]
        About { name: String }
}

