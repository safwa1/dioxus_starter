mod en;
mod ar;

use dioxus::prelude::*;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use crate::state::app_state::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Arabic,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

#[derive(Error, Debug)]
pub enum I18nError {
    #[error("Translation key not found: {0}")]
    MissingKey(String),
}

pub type I18nResult<T> = Result<T, I18nError>;

pub struct Translator {
    language: Language,
}

impl Translator {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn get_language(&self) -> Language {
        self.language
    }

    pub fn text_or(&self, key: &str, default: &'static str) -> &'static str {
        self.get(key).unwrap_or(default)
    }

    pub fn text(&self, key: &str) -> &'static str {
        self.text_or(key, "")
    }

    pub fn get(&self, key: &str) -> I18nResult<&'static str> {
        match self.language {
            Language::Arabic => ar::STRINGS
                .get(key)
                .copied()
                .ok_or_else(|| I18nError::MissingKey(key.to_string())),
            Language::English => en::STRINGS
                .get(key)
                .copied()
                .ok_or_else(|| I18nError::MissingKey(key.to_string())),
        }
    }

    pub fn get_formatted(&self, key: &str, args: &[&str]) -> I18nResult<String> {
        let template = self.get(key)?;
        let mut result = template.to_string();
        
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("{{{}}}", i), arg);
        }
        
        Ok(result)
    }

    pub fn formatted_text(&self, key: &str, args: &[&str]) -> String {
        self.get_formatted(key, args).unwrap_or("".to_string())
    }
}

pub fn use_translator() -> Translator {
    let state = use_context::<Signal<AppState>>();
    let language = state().language();
    Translator::new(language)
}
