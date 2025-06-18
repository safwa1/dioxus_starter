use dioxus::{hooks::use_context, signals::Signal};
use dioxus_desktop::use_window;

use crate::{localization::Language, models::Theme};

#[derive(Clone)]
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
    }

    pub fn toggle_language(&mut self) {
        self.language = match self.language {
            Language::Arabic => Language::English,
            Language::English => Language::Arabic
        };
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn is_dark(&self) -> bool {
        self.theme == Theme::Dark
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
        self.apply_theme();
    }

    fn apply_theme(&self) {
        let window = use_window();
        match self.theme {
            Theme::Dark => window.webview.evaluate_script(
                r#"document.documentElement.classList.add('dark')"#
            ),
            Theme::Light => window.webview.evaluate_script(
                r#"document.documentElement.classList.remove('dark')"#
            ),
        }.unwrap();
    }
}


pub fn use_app_state() -> Signal<AppState> {
    use_context::<Signal<AppState>>()
}