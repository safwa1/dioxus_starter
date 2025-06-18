use phf::phf_map;

pub static STRINGS: phf::Map<&'static str, &'static str> = phf_map! {
    "app_title" => "Dioxus Starter Kit",
    "home_page_label" => "Home Page",
    "about_page_label" => "About Page",
    "back_to_home" => "Back To Home",
    "go_to_about" => "Go to about page.",
    "welcome_message" => "Welcome {0}",
    "switch_language" => "Switch Language",
    "switch_theme" => "Switch Theme",
};
