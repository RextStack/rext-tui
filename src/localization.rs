//! Localization system for the TUI
//!
//! A simple localization system for Rext TUI.
//!
//! Each localization file is split into three categories:
//! - ui
//! - messages
//! - keys
//!
//! Here is a small portion of english's toml (localization/en.toml) as an example
//! ```toml
//! [ui]
//! add_api_endpoint = "Add API endpoint"
//! add_api_endpoint_shortcut = " (e)"
//! theme_label = "Theme: "
//! theme_shortcut = " (t)"
//! api_endpoint_name_prompt = "API endpoint name:"
//! input_cursor = "_"
//!
//! [messages]
//! quit_instruction_prefix = "Press "
//! quit_instruction_middle = " or "
//! quit_instruction_suffix = " to quit"
//!
//! [keys]
//! add_endpoint = "e"
//! toggle_theme = "t"
//! quit = "q"
//! quit_combo = "Ctrl+C"
//! escape = "Esc"
//! enter = "Enter"
//! backspace = "Backspace"
//! ```
//!
//! ui is for general text display on the user interface such as instructions and input labels.
//!
//! messages are intended for storing text that prompts the user, not strictly tied to one portion of the UI
//!
//! keys are for both displaying and controlling which key should be pressed on the keyboard for an action.
//! Each key entry serves dual purpose - both for display and actual key binding.
use crossterm::event::{KeyCode, KeyModifiers};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use crate::error::RextTuiError;

/// Stores the localized texts for the TUI from the localization directory
#[derive(Debug, Deserialize, Clone)]
pub struct LocalizedTexts {
    pub ui: HashMap<String, String>,
    pub messages: HashMap<String, String>,
    pub keys: HashMap<String, String>,
}

/// The localization system for the TUI
pub struct Localization {
    texts: LocalizedTexts,
    fallback_texts: LocalizedTexts, // English as fallback
}

impl Localization {
    /// Creates a new localization system for the TUI, english is the fallback
    pub fn new(lang: &str) -> Result<Self, RextTuiError> {
        let fallback_texts = Self::load_language("en")?;
        let texts = if lang == "en" {
            fallback_texts.clone()
        } else {
            Self::load_language(lang).unwrap_or_else(|_| fallback_texts.clone())
        };

        Ok(Self {
            texts,
            fallback_texts,
        })
    }

    /// Reloads the localization system with a new language
    pub fn reload(&mut self, lang: &str) -> Result<(), RextTuiError> {
        let texts = if lang == "en" {
            self.fallback_texts.clone()
        } else {
            Self::load_language(lang).unwrap_or_else(|_| self.fallback_texts.clone())
        };
        self.texts = texts;
        Ok(())
    }

    /// Loads the localized texts for the TUI from the localization directory
    fn load_language(lang: &str) -> Result<LocalizedTexts, RextTuiError> {
        let path = format!("localization/{}.toml", lang);
        let content = fs::read_to_string(&path).map_err(|e| RextTuiError::ReadConfigFile(e))?;
        toml::from_str(&content).map_err(|e| RextTuiError::ConfigError(e))
    }

    /// Gets the localized text for the TUI
    /// section: The section of the text to get (ui, messages, keys)
    /// key: The key of the text to get (not keyboard key, the key in the toml file)
    /// Returns the localized text
    ///
    /// # Example
    ///
    /// ```rust
    /// use rext_tui::localization::Localization;
    /// let localization = Localization::new("en").unwrap();
    /// let text = localization.get("ui", "add_api_endpoint");
    /// assert_eq!(text, "Add API endpoint");
    /// ```
    pub fn get(&self, section: &str, key: &str) -> &str {
        let section_map = match section {
            "ui" => &self.texts.ui,
            "messages" => &self.texts.messages,
            "keys" => &self.texts.keys,
            _ => return "Unknown section",
        };

        section_map
            .get(key)
            .or_else(|| {
                // Fallback to English if key missing
                let fallback_section = match section {
                    "ui" => &self.fallback_texts.ui,
                    "messages" => &self.fallback_texts.messages,
                    "keys" => &self.fallback_texts.keys,
                    _ => return None,
                };
                fallback_section.get(key)
            })
            .map(|s| s.as_str())
            .unwrap_or("Missing text")
    }

    /// Convenience method for UI texts
    pub fn ui(&self, key: &str) -> &str {
        self.get("ui", key)
    }

    /// Convenience method for message texts
    pub fn msg(&self, key: &str) -> &str {
        self.get("messages", key)
    }

    /// Convenience method for key texts
    pub fn key(&self, key: &str) -> &str {
        self.get("keys", key)
    }

    /// Gets the actual key code for a given action
    pub fn get_key_code(&self, action: &str) -> Option<(KeyModifiers, KeyCode)> {
        let key_str = self.key(action);
        Self::parse_key_string(key_str)
    }

    /// Parses a key string into KeyModifiers and KeyCode
    /// Examples: "q" -> (KeyModifiers::NONE, KeyCode::Char('q'))
    ///          "Ctrl+C" -> (KeyModifiers::CONTROL, KeyCode::Char('C'))
    ///          "Esc" -> (KeyModifiers::NONE, KeyCode::Esc)
    fn parse_key_string(key_str: &str) -> Option<(KeyModifiers, KeyCode)> {
        if key_str.contains("Ctrl+") {
            let key_part = key_str.strip_prefix("Ctrl+")?;
            if key_part.len() == 1 {
                let ch = key_part.chars().next()?;
                Some((KeyModifiers::CONTROL, KeyCode::Char(ch)))
            } else {
                None
            }
        } else {
            match key_str {
                "Esc" => Some((KeyModifiers::NONE, KeyCode::Esc)),
                "Enter" => Some((KeyModifiers::NONE, KeyCode::Enter)),
                "Backspace" => Some((KeyModifiers::NONE, KeyCode::Backspace)),
                single_char if single_char.len() == 1 => {
                    let ch = single_char.chars().next()?;
                    Some((KeyModifiers::NONE, KeyCode::Char(ch)))
                }
                _ => None,
            }
        }
    }

    /// Checks if the given key event matches the configured key for an action
    pub fn matches_key(&self, action: &str, modifiers: KeyModifiers, code: KeyCode) -> bool {
        if let Some((expected_modifiers, expected_code)) = self.get_key_code(action) {
            // For character keys, check both lowercase and uppercase
            match (expected_code, code) {
                (KeyCode::Char(expected), KeyCode::Char(actual)) => {
                    expected_modifiers == modifiers
                        && (expected == actual
                            || expected.to_ascii_lowercase() == actual.to_ascii_lowercase())
                }
                _ => expected_modifiers == modifiers && expected_code == code,
            }
        } else {
            false
        }
    }
}
