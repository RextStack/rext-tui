//! Configuration system for Rext TUI
//!
//! Handles config files for the TUI application, including themes,
//! localization settings, and user preferences.
//!
//! ## Configuration Strategy
//!
//! This module uses a hybrid approach:
//! 1. **Default configs embedded in binary** - Always available, zero-config startup
//! 2. **User overrides in ~/.rext/** - Optional customization for power users
//!
//! ## Configuration Files
//!
//! ### Embedded Defaults
//! - Main config with themes and localization metadata (embedded)
//! - English and French localization files (embedded)
//!
//! ### User Directory (`~/.rext/`)
//! - `rext_tui.toml` - User's custom config (overrides embedded default)
//! - `current_theme.toml` - User's selected theme
//! - `current_localization.toml` - User's selected language
//!
//! ### Main Config Format
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
//! ## Usage
//!
//! ```rust
//! use rext_tui::config::{load_config, load_current_theme, get_available_themes};
//!
//! // Load configuration (user override or embedded default)
//! let config = load_config().unwrap();
//!
//! // Get current theme with fallback to default
//! let theme_name = load_current_theme().unwrap_or_else(|_| "rust".to_string());
//!
//! // Get available themes
//! let themes = get_available_themes().unwrap();
//! ```
//!
//! ## Error Handling
//!
//! Falls back to embedded defaults when user configs are invalid or missing.
//! This ensures the app always works even with broken user customizations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::error::RextTuiError;

// Embedded default configurations
const DEFAULT_CONFIG: &str = include_str!("../config/rext_tui.toml");
const EN_LOCALIZATION: &str = include_str!("../localization/en.toml");
const FR_LOCALIZATION: &str = include_str!("../localization/fr.toml");

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

/// Main configuration structure loaded from config files
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

/// Stores the current theme name for the TUI in current_theme.toml
#[derive(Deserialize, Serialize)]
pub struct CurrentTheme {
    pub current_theme: String,
}

/// Stores the current localization for the TUI in current_localization.toml
#[derive(Deserialize, Serialize)]
pub struct CurrentLocalization {
    pub current_localization: String,
}

/// Gets the rext configuration directory path (~/.rext/)
///
/// Creates the directory if it doesn't exist.
fn get_rext_config_dir() -> Result<PathBuf, RextTuiError> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        RextTuiError::ReadConfigFile(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find home directory",
        ))
    })?;

    let rext_dir = home_dir.join(".rext");

    // Create directory if it doesn't exist
    if !rext_dir.exists() {
        fs::create_dir_all(&rext_dir).map_err(|e| RextTuiError::WriteConfigFile(e))?;
    }

    Ok(rext_dir)
}

/// Gets the path for current theme config file
fn get_current_theme_path() -> Result<PathBuf, RextTuiError> {
    Ok(get_rext_config_dir()?.join("current_theme.toml"))
}

/// Gets the path for current localization config file
fn get_current_localization_path() -> Result<PathBuf, RextTuiError> {
    Ok(get_rext_config_dir()?.join("current_localization.toml"))
}

/// Gets the path for user's custom config file
fn get_user_config_path() -> Result<PathBuf, RextTuiError> {
    Ok(get_rext_config_dir()?.join("rext_tui.toml"))
}

/// Loads the main configuration
///
/// Checks for user config in ~/.rext/rext_tui.toml first, falls back to embedded default.
/// This ensures the app always works even if user config is missing or invalid.
///
/// # Returns
///
/// - `Ok(Config)`: Successfully loaded configuration
/// - `Err(RextTuiError)`: Only fails if embedded config is invalid (should never happen)
pub fn load_config() -> Result<Config, RextTuiError> {
    // Try to load user config first
    if let Ok(user_config_path) = get_user_config_path() {
        if user_config_path.exists() {
            if let Ok(contents) = fs::read_to_string(&user_config_path) {
                if let Ok(config) = toml::from_str::<Config>(&contents) {
                    return Ok(config);
                }
                // If user config is invalid, we'll fall back to embedded default
                // Could log a warning here in the future
            }
        }
    }

    // Fall back to embedded default config
    let config: Config =
        toml::from_str(DEFAULT_CONFIG).map_err(|e| RextTuiError::ConfigError(e))?;
    Ok(config)
}

/// Loads the current theme name from ~/.rext/current_theme.toml
///
/// # Returns
///
/// - `Ok(String)`: The current theme name (e.g., "rust", "dracula")
/// - `Err(RextTuiError)`: File not found, parse error, or I/O error
pub fn load_current_theme() -> Result<String, RextTuiError> {
    let theme_path = get_current_theme_path()?;
    let contents = fs::read_to_string(&theme_path).map_err(|e| RextTuiError::ReadConfigFile(e))?;
    let theme_config: CurrentTheme =
        toml::from_str(&contents).map_err(|e| RextTuiError::ConfigError(e))?;
    Ok(theme_config.current_theme)
}

/// Saves the current theme name to ~/.rext/current_theme.toml
///
/// # Arguments
///
/// * `theme_name` - The name of the theme to save
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
    let theme_path = get_current_theme_path()?;
    fs::write(&theme_path, contents).map_err(|e| RextTuiError::WriteConfigFile(e))?;
    Ok(())
}

/// Loads the selected theme colors from the config
///
/// # Arguments
///
/// * `theme_name` - The name of the theme to load colors from
///
/// # Returns
///
/// - `Ok(Colors)`: The colors for the selected theme
/// - `Err(RextTuiError)`: Theme not found or config error
pub fn load_theme_colors(theme_name: &str) -> Result<Colors, RextTuiError> {
    let config = load_config()?;

    config
        .themes
        .get(theme_name)
        .cloned()
        .ok_or_else(|| RextTuiError::ThemeNotFound(theme_name.to_string()))
}

/// Gets the available themes from the config
///
/// # Returns
///
/// - `Ok(Vec<String>)`: A list of available theme names
/// - `Err(RextTuiError)`: Config loading error
pub fn get_available_themes() -> Result<Vec<String>, RextTuiError> {
    let config = load_config()?;
    let mut themes: Vec<String> = config.themes.keys().cloned().collect();
    themes.sort();
    Ok(themes)
}

/// Loads the current language from ~/.rext/current_localization.toml
///
/// # Returns
///
/// - `Ok(String)`: The current language code (e.g., "en", "fr")
/// - `Err(RextTuiError)`: File not found, parse error, or I/O error
pub fn load_current_language() -> Result<String, RextTuiError> {
    let localization_path = get_current_localization_path()?;
    let contents =
        fs::read_to_string(&localization_path).map_err(|e| RextTuiError::ReadConfigFile(e))?;
    let localization_config: CurrentLocalization =
        toml::from_str(&contents).map_err(|e| RextTuiError::ConfigError(e))?;
    Ok(localization_config.current_localization)
}

/// Saves the current language to ~/.rext/current_localization.toml
///
/// # Arguments
///
/// * `language` - The language code to save
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
    let localization_path = get_current_localization_path()?;
    fs::write(&localization_path, contents).map_err(|e| RextTuiError::WriteConfigFile(e))?;
    Ok(())
}

/// Gets the available languages from the config
///
/// # Returns
///
/// - `Ok(Vec<String>)`: A list of available language codes
/// - `Err(RextTuiError)`: Config loading error
pub fn get_available_languages() -> Result<Vec<String>, RextTuiError> {
    let config = load_config()?;
    let mut languages: Vec<String> = config.localization.keys().cloned().collect();
    languages.sort();
    Ok(languages)
}

/// Gets the available languages with their display names from the config
///
/// # Returns
///
/// - `Ok(Vec<(String, String)>)`: A list of available languages with their display names
/// - `Err(RextTuiError)`: Config loading error
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

/// Loads localization content for a specific language
///
/// Checks for user localization files first, falls back to embedded defaults.
///
/// # Arguments
///
/// * `language_code` - The language code (e.g., "en", "fr")
///
/// # Returns
///
/// - `Ok(String)`: The localization file content
/// - `Err(RextTuiError)`: Language not supported
pub fn load_localization_content(language_code: &str) -> Result<String, RextTuiError> {
    // Try user localization file first
    if let Ok(rext_dir) = get_rext_config_dir() {
        let user_localization_path = rext_dir
            .join("localization")
            .join(format!("{}.toml", language_code));
        if user_localization_path.exists() {
            if let Ok(contents) = fs::read_to_string(&user_localization_path) {
                // Validate that it's valid TOML before returning
                if toml::from_str::<toml::Value>(&contents).is_ok() {
                    return Ok(contents);
                }
            }
        }
    }

    // Fall back to embedded localization
    let content = match language_code {
        "en" => EN_LOCALIZATION,
        "fr" => FR_LOCALIZATION,
        _ => EN_LOCALIZATION, // Default to English for unsupported languages
    };

    Ok(content.to_string())
}
