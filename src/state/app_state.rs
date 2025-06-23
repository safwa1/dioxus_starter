use dioxus::{hooks::use_context, signals::Signal};
use dioxus_desktop::use_window;
use serde::{Deserialize, Serialize};
use crate::{localization::Language, models::Theme};

#[derive(Clone, Serialize, Deserialize)]
pub struct AppState {
    language: Language,
    theme: Theme,
}

impl Default for AppState {
    fn default() -> Self {
        Self { language: Language::default(), theme: Theme::default() }
    }
}

impl AppState {
    pub fn language(&self) -> Language {
        self.language
    }

    pub fn theme(&self) -> Theme {
        self.theme
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
        self.save();
    }

    pub fn toggle_language(&mut self) {
        let language = match self.language {
            Language::Arabic => Language::English,
            Language::English => Language::Arabic
        };
        self.set_language(language);
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
        self.save();
    }

    pub fn is_dark(&self) -> bool {
        self.theme == Theme::Dark
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
        self.apply_theme(self.theme);
        println!("theme changed to {:?}", self.theme);
        self.save();
    }

    pub fn apply_theme(&self, theme: Theme) {
        let window = use_window();
        match theme {
            Theme::Dark => {
                window.set_theme(Some(dioxus_desktop::tao::window::Theme::Dark));
                window.webview.evaluate_script(r#"document.documentElement.classList.add('dark')"#)
            },
            Theme::Light => {
                window.set_theme(Some(dioxus_desktop::tao::window::Theme::Light));
                window.webview.evaluate_script(r#"document.documentElement.classList.remove('dark')"#)
            },
        }.unwrap();
    }

    pub fn load() -> AppState {
        // confy will automatically create the directory and file if they don't exist
        // and return the default value if the file is empty or corrupted.
        const APP_NAME: &str = env!("CARGO_PKG_NAME");
        confy::load(APP_NAME, None).unwrap_or_default()
    }

    pub fn save(&self) {
        // confy will save the state to the appropriate config directory
        const APP_NAME: &str = env!("CARGO_PKG_NAME");
        if let Err(e) = confy::store(APP_NAME, None, self) {
            eprintln!("Failed to save app state: {:?}", e);
        }
    }
}


pub fn use_app_state() -> Signal<AppState> {
    use_context::<Signal<AppState>>()
}