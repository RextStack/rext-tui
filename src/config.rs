//! Configuration system for Rext TUI
//!
//! Handles config files for the TUI application, including themes,
//! localization settings, and user preferences.
//!
//! ## Configuration Files
//!
//! The TUI uses several TOML configuration files located in the `config/` directory:
//!
//! ### Main Config (`config/rext_tui.toml`)
//!
//! Contains theme definitions and available localizations:
//!
//! ```toml
//! # Theme definitions with RGB color values
//! [themes.rust]
//! text = { r = 204, g = 205, b = 204 }
//! primary = { r = 255, g = 107, b = 53 }
//! background = { r = 26, g = 26, b = 26 }
//!
//! # Localizations
//! [localization.en]
//! language = "en"
//! display = "English"
//! ```
//!
//! ### Current Theme, untracked (`config/current_theme.toml`)
//!
//! Stores the user's selected theme:
//!
//! ```toml
//! current_theme = "rust"
//! ```
//!
//! ### Current Localization, untracked (`config/current_localization.toml`)
//!
//! Stores the user's selected language:
//!
//! ```toml
//! current_localization = "en"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use rext_tui::config::{load_config, load_current_theme, get_available_themes};
//!
//! // Load main configuration
//! let config = load_config().unwrap();
//!
//! // Get current theme
//! let theme_name = load_current_theme().unwrap();
//!
//! // Get available themes
//! let themes = get_available_themes().unwrap();
//! ```
//!
//! ## Color System
//!
//! Each theme defines three RGB colors:
//! - `primary`: Accent color for highlights and focus
//! - `text`: Regular text color
//! - `background`: Background color
//!
//! Fallback tosensible defaults when configuration loading fails.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::error::RextTuiError;

/// RGB color value for theme configuration
///
/// Used to define colors in theme configuration files. Each component
/// should be a value between 0 and 255.
///
/// # Example
///
/// ```toml
/// primary = { r = 255, g = 107, b = 53 }  # Orange color
/// ```
#[derive(Deserialize, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Main configuration structure loaded from `config/rext_tui.toml`
///
/// Contains all theme definitions and available localizations for the TUI.
///
/// # Example Structure
///
/// ```toml
/// [themes.rust]
/// text = { r = 204, g = 205, b = 204 }
/// primary = { r = 255, g = 107, b = 53 }
/// background = { r = 26, g = 26, b = 26 }
///
/// [localization.en]
/// language = "en"
/// display = "English"
/// ```
#[derive(Deserialize)]
pub struct Config {
    pub themes: HashMap<String, Colors>,
    pub localization: HashMap<String, LocalizationConfig>,
}

/// Localization configuration for a specific language
///
/// Defines the language code and display name for UI presentation.
///
/// # Fields
///
/// - `language`: The language code (e.g., "en", "fr")
/// - `display`: The display name (e.g., "English", "French")
#[derive(Deserialize)]
pub struct LocalizationConfig {
    pub language: String,
    pub display: String,
}

/// Color scheme definition for a theme
///
/// Defines the three main colors used throughout the TUI interface.
///
/// # Color Usage
///
/// - `primary`: Accent color for highlights, borders, and interactive elements
/// - `text`: Regular text color for most content
/// - `background`: Background color for the entire application
#[derive(Deserialize, Clone)]
pub struct Colors {
    pub primary: Rgb,
    pub text: Rgb,
    pub background: Rgb,
}

/// Stores the current theme name for the TUI from current_theme.toml
#[derive(Deserialize, Serialize)]
pub struct CurrentTheme {
    pub current_theme: String,
}

/// Stores the current localization for the TUI from current_localization.toml
#[derive(Deserialize, Serialize)]
pub struct CurrentLocalization {
    pub current_localization: String,
}

/// Path to the main config file
pub const CONFIG_PATH: &str = "config/rext_tui.toml";

/// Path to the current theme file
pub const CURRENT_THEME_PATH: &str = "config/current_theme.toml";

/// Path to the current localization file
pub const CURRENT_LOCALIZATION_PATH: &str = "config/current_localization.toml";

/// Loads the main configuration from `config/rext_tui.toml`
///
/// This function reads and parses the main configuration file containing
/// theme definitions and available localizations.
///
/// # Returns
///
/// - `Ok(Config)`: Successfully loaded configuration
/// - `Err(RextTuiError)`: File not found, parse error, or I/O error
pub fn load_config() -> Result<Config, RextTuiError> {
    let contents: String =
        fs::read_to_string(CONFIG_PATH).map_err(|e| RextTuiError::ReadConfigFile(e))?;
    let config: Config = toml::from_str(&contents).map_err(|e| RextTuiError::ConfigError(e))?;
    Ok(config)
}

/// Loads the current theme name from `config/current_theme.toml`
///
/// Returns the name of the currently selected theme.
///
/// # Returns
///
/// - `Ok(String)`: The current theme name (e.g., "rust", "dracula")
/// - `Err(RextTuiError)`: File not found, parse error, or I/O error
pub fn load_current_theme() -> Result<String, RextTuiError> {
    let contents =
        fs::read_to_string(CURRENT_THEME_PATH).map_err(|e| RextTuiError::ReadConfigFile(e))?;
    let theme_config: CurrentTheme =
        toml::from_str(&contents).map_err(|e| RextTuiError::ConfigError(e))?;
    Ok(theme_config.current_theme)
}

/// Saves the current theme name to `config/current_theme.toml`
///
/// Persists the user's theme selection to the configuration file.
///
/// # Arguments
///
/// * `theme_name` - The name of the theme to save (must exist in main config)
///
/// # Returns
///
/// - `Ok(())`: Theme successfully saved
/// - `Err(RextTuiError)`: Serialization error or I/O error
pub fn save_current_theme(theme_name: &str) -> Result<(), RextTuiError> {
    let theme_config = CurrentTheme {
        current_theme: theme_name.to_string(),
    };
    let contents = toml::to_string(&theme_config).map_err(|e| RextTuiError::SerializeError(e))?;
    fs::write("config/current_theme.toml", contents)
        .map_err(|e| RextTuiError::WriteConfigFile(e))?;
    Ok(())
}

/// Loads the selected theme colors from the main config file
///
/// # Arguments
///
/// * `theme_name` - The name of the theme to load colors from
///
/// # Returns
///
/// - `Ok(Colors)`: The colors for the selected theme
pub fn load_theme_colors(theme_name: &str) -> Result<Colors, RextTuiError> {
    let config = load_config()?;

    config
        .themes
        .get(theme_name)
        .cloned()
        .ok_or_else(|| RextTuiError::ThemeNotFound(theme_name.to_string()))
}

/// Gets the available themes from the main config file
///
/// # Returns
///
/// - `Ok(Vec<String>)`: A list of available theme names
pub fn get_available_themes() -> Result<Vec<String>, RextTuiError> {
    let config = load_config()?;
    let mut themes: Vec<String> = config.themes.keys().cloned().collect();
    themes.sort();
    Ok(themes)
}

/// Loads the current language from the current localization file
///
/// # Returns
///
/// - `Ok(String)`: The current language code (e.g., "en", "fr")
pub fn load_current_language() -> Result<String, RextTuiError> {
    let contents = fs::read_to_string(CURRENT_LOCALIZATION_PATH)
        .map_err(|e| RextTuiError::ReadConfigFile(e))?;
    let localization_config: LocalizationConfig =
        toml::from_str(&contents).map_err(|e| RextTuiError::ConfigError(e))?;
    Ok(localization_config.language)
}

/// Saves the current language to the current localization file
///
/// # Arguments
///
/// * `language` - The language code to save (must exist in main config)
///
/// # Returns
///
/// - `Ok(())`: Language successfully saved
/// - `Err(RextTuiError)`: Serialization error or I/O error
pub fn save_current_language(language: &str) -> Result<(), RextTuiError> {
    let localization_config = CurrentLocalization {
        current_localization: language.to_string(),
    };
    let contents =
        toml::to_string(&localization_config).map_err(|e| RextTuiError::SerializeError(e))?;
    fs::write(CURRENT_LOCALIZATION_PATH, contents).map_err(|e| RextTuiError::WriteConfigFile(e))?;
    Ok(())
}

/// Gets the available languages from the main config file
///
/// # Returns
///
/// - `Ok(Vec<String>)`: A list of available language codes
pub fn get_available_languages() -> Result<Vec<String>, RextTuiError> {
    let config = load_config()?;
    let mut languages: Vec<String> = config.localization.keys().cloned().collect();
    languages.sort();
    Ok(languages)
}

/// Gets the available languages with their display names from the main config file
///
/// # Returns
///
/// - `Ok(Vec<(String, String)>)`: A list of available languages with their display names
pub fn get_available_languages_with_display() -> Result<Vec<(String, String)>, RextTuiError> {
    let config = load_config()?;
    let mut languages: Vec<(String, String)> = config
        .localization
        .iter()
        .map(|(key, value)| (key.clone(), value.display.clone()))
        .collect();
    languages.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(languages)
}
