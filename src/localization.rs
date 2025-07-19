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
//! up = "Up"
//! down = "Down"
//! ```
//!
//! ## UI and Messages
//! ui is for general text display on the user interface such as instructions and input labels.
//! messages are intended for storing text that prompts the user, not strictly tied to one portion of the UI
//!
//! ## Keys
//! keys are for both displaying and controlling which key should be pressed on the keyboard for an action.
//! Each key entry serves dual purpose - both for display and actual key binding.
//!
//! ## Supported Key Formats
//! The localization system supports a wide range of key formats (case-insensitive):
//! - **Single characters**: "a", "q", "1", "2"
//! - **Special keys**: "Esc"/"Escape", "Enter"/"Return", "Backspace"/"Back", "Tab", "Delete"/"Del", "Insert"/"Ins"
//! - **Arrow keys**: "Up", "Down", "Left", "Right", "UpArrow", "DownArrow", "LeftArrow", "RightArrow"
//! - **Navigation keys**: "Home", "End", "PageUp"/"PgUp", "PageDown"/"PgDn"
//! - **Function keys**: "F1", "F2", ..., "F12"
//! - **Modifier combinations**: "Ctrl+C", "Shift+Tab", "Alt+Enter", "Control+A"
//!
//! The system validates all key bindings on startup and will warn about invalid key strings.
use crossterm::event::{KeyCode, KeyModifiers};
use serde::Deserialize;
use std::collections::HashMap;

use crate::config;
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

        let localization = Self {
            texts,
            fallback_texts,
        };

        // Validate key bindings on creation
        localization.validate_key_bindings();

        Ok(localization)
    }

    /// Reloads the localization system with a new language
    pub fn reload(&mut self, lang: &str) -> Result<(), RextTuiError> {
        let texts = if lang == "en" {
            self.fallback_texts.clone()
        } else {
            Self::load_language(lang).unwrap_or_else(|_| self.fallback_texts.clone())
        };
        self.texts = texts;

        // Validate key bindings after reload
        self.validate_key_bindings();

        Ok(())
    }

    /// Validates all key bindings in the current localization
    /// Prints warnings for any keys that cannot be parsed
    pub fn validate_key_bindings(&self) {
        let mut invalid_keys = Vec::new();

        for (action, key_str) in &self.texts.keys {
            if Self::parse_key_string(key_str).is_none() {
                invalid_keys.push((action.clone(), key_str.clone()));
            }
        }

        if !invalid_keys.is_empty() {
            eprintln!(
                "Warning: Found {} invalid key binding(s) in localization:",
                invalid_keys.len()
            );
            for (action, key_str) in invalid_keys {
                eprintln!("  - Action '{}': Invalid key string '{}'", action, key_str);
            }
            eprintln!("These key bindings will not work. Please check your localization files.");
        }
    }

    /// Gets a list of all supported key string formats for documentation
    pub fn get_supported_key_formats() -> Vec<&'static str> {
        vec![
            // Single characters
            "a",
            "q",
            "1",
            "2",
            // Special keys
            "Esc",
            "Escape",
            "Enter",
            "Return",
            "Backspace",
            "Back",
            "Tab",
            "Delete",
            "Del",
            "Insert",
            "Ins",
            // Arrow keys
            "Up",
            "Down",
            "Left",
            "Right",
            "UpArrow",
            "DownArrow",
            "LeftArrow",
            "RightArrow",
            // Navigation keys
            "Home",
            "End",
            "PageUp",
            "PgUp",
            "PageDown",
            "PgDn",
            // Function keys
            "F1",
            "F2",
            "F3",
            "F4",
            "F5",
            "F6",
            "F7",
            "F8",
            "F9",
            "F10",
            "F11",
            "F12",
            // Modifier combinations
            "Ctrl+C",
            "Shift+Tab",
            "Alt+Enter",
            "Control+A",
            "Shift+F1",
        ]
    }

    /// Loads the localized texts for the TUI using the config system
    ///
    /// This loads from user overrides first, then falls back to embedded defaults.
    fn load_language(lang: &str) -> Result<LocalizedTexts, RextTuiError> {
        let content = config::load_localization_content(lang)?;
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
    /// Supports common key formats including:
    /// - Single characters: "q", "a", "1"
    /// - Special keys: "Esc", "Enter", "Backspace", "Tab", "Delete"
    /// - Arrow keys: "Up", "Down", "Left", "Right"
    /// - Navigation: "Home", "End", "PageUp", "PageDown"
    /// - Function keys: "F1", "F2", ..., "F12"
    /// - Modifiers: "Ctrl+C", "Shift+Tab", "Alt+Enter"
    /// - Case insensitive: "up", "UP", "Up" all work
    fn parse_key_string(key_str: &str) -> Option<(KeyModifiers, KeyCode)> {
        let key_str = key_str.trim();

        // Handle modifier combinations
        if key_str.contains('+') {
            return Self::parse_modified_key(key_str);
        }

        // Handle single keys (case-insensitive)
        let normalized = key_str.to_lowercase();
        match normalized.as_str() {
            // Special keys
            "esc" | "escape" => Some((KeyModifiers::NONE, KeyCode::Esc)),
            "enter" | "return" => Some((KeyModifiers::NONE, KeyCode::Enter)),
            "backspace" | "back" => Some((KeyModifiers::NONE, KeyCode::Backspace)),
            "tab" => Some((KeyModifiers::NONE, KeyCode::Tab)),
            "delete" | "del" => Some((KeyModifiers::NONE, KeyCode::Delete)),
            "insert" | "ins" => Some((KeyModifiers::NONE, KeyCode::Insert)),

            // Arrow keys
            "up" | "uparrow" => Some((KeyModifiers::NONE, KeyCode::Up)),
            "down" | "downarrow" => Some((KeyModifiers::NONE, KeyCode::Down)),
            "left" | "leftarrow" => Some((KeyModifiers::NONE, KeyCode::Left)),
            "right" | "rightarrow" => Some((KeyModifiers::NONE, KeyCode::Right)),

            // Navigation keys
            "home" => Some((KeyModifiers::NONE, KeyCode::Home)),
            "end" => Some((KeyModifiers::NONE, KeyCode::End)),
            "pageup" | "pgup" => Some((KeyModifiers::NONE, KeyCode::PageUp)),
            "pagedown" | "pgdn" => Some((KeyModifiers::NONE, KeyCode::PageDown)),

            // Function keys
            "f1" => Some((KeyModifiers::NONE, KeyCode::F(1))),
            "f2" => Some((KeyModifiers::NONE, KeyCode::F(2))),
            "f3" => Some((KeyModifiers::NONE, KeyCode::F(3))),
            "f4" => Some((KeyModifiers::NONE, KeyCode::F(4))),
            "f5" => Some((KeyModifiers::NONE, KeyCode::F(5))),
            "f6" => Some((KeyModifiers::NONE, KeyCode::F(6))),
            "f7" => Some((KeyModifiers::NONE, KeyCode::F(7))),
            "f8" => Some((KeyModifiers::NONE, KeyCode::F(8))),
            "f9" => Some((KeyModifiers::NONE, KeyCode::F(9))),
            "f10" => Some((KeyModifiers::NONE, KeyCode::F(10))),
            "f11" => Some((KeyModifiers::NONE, KeyCode::F(11))),
            "f12" => Some((KeyModifiers::NONE, KeyCode::F(12))),

            // Single character keys
            single_char if single_char.len() == 1 => {
                let ch = key_str.chars().next()?; // Use original case for character
                Some((KeyModifiers::NONE, KeyCode::Char(ch)))
            }

            // Unknown key
            _ => {
                eprintln!("Warning: Unknown key string '{}' in localization", key_str);
                None
            }
        }
    }

    /// Parses modified key combinations like "Ctrl+C", "Shift+Tab", "Alt+Enter"
    fn parse_modified_key(key_str: &str) -> Option<(KeyModifiers, KeyCode)> {
        let parts: Vec<&str> = key_str.split('+').collect();
        if parts.len() != 2 {
            eprintln!(
                "Warning: Invalid key combination '{}' in localization",
                key_str
            );
            return None;
        }

        let modifier_str = parts[0].trim().to_lowercase();
        let key_part = parts[1].trim();

        let modifiers = match modifier_str.as_str() {
            "ctrl" | "control" => KeyModifiers::CONTROL,
            "shift" => KeyModifiers::SHIFT,
            "alt" => KeyModifiers::ALT,
            _ => {
                eprintln!(
                    "Warning: Unknown modifier '{}' in key combination '{}'",
                    modifier_str, key_str
                );
                return None;
            }
        };

        // Parse the key part (recursively, but without modifiers)
        if let Some((_, key_code)) = Self::parse_key_string(key_part) {
            Some((modifiers, key_code))
        } else {
            eprintln!(
                "Warning: Invalid key '{}' in combination '{}'",
                key_part, key_str
            );
            None
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
