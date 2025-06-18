use phf::phf_map;

pub static STRINGS: phf::Map<&'static str, &'static str> = phf_map! {
    "app_title" => "ديوكسوس ستارتر",
    "home_page_label" => "الرئيسية",
    "about_page_label" => "عن البرنامج",
    "back_to_home" => "عودة الى الرئيسية",
    "go_to_about" => "فتح صفحة عن البرنامج.",
    "welcome_message" => "مرحبا بك يا {0}",
    "switch_language" => "تبديل اللغة",
    "switch_theme" => "تبديل الثيم",
};