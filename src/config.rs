use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::error::RextTuiError;

/// Simple RGB color struct
#[derive(Deserialize, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Stores the config values for the TUI from rext_tui.toml
#[derive(Deserialize)]
pub struct Config {
    pub themes: HashMap<String, Colors>,
    pub localization: HashMap<String, LocalizationConfig>,
}

/// Stores the localization config
#[derive(Deserialize)]
pub struct LocalizationConfig {
    pub language: String,
    pub display: String,
}

/// Stores the color values for the TUI from rext_tui.toml
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

const CONFIG_PATH: &str = "config/rext_tui.toml";
const CURRENT_THEME_PATH: &str = "config/current_theme.toml";
const CURRENT_LOCALIZATION_PATH: &str = "config/current_localization.toml";

/// Loads the config values for the TUI from rext_tui.toml
pub fn load_config() -> Result<Config, RextTuiError> {
    let contents: String =
        fs::read_to_string(CONFIG_PATH).map_err(|e| RextTuiError::ReadConfigFile(e))?;
    let config: Config = toml::from_str(&contents).map_err(|e| RextTuiError::ConfigError(e))?;
    Ok(config)
}

/// Loads the current theme name for the TUI from current_theme.toml
pub fn load_current_theme() -> Result<String, RextTuiError> {
    let contents =
        fs::read_to_string(CURRENT_THEME_PATH).map_err(|e| RextTuiError::ReadConfigFile(e))?;
    let theme_config: CurrentTheme =
        toml::from_str(&contents).map_err(|e| RextTuiError::ConfigError(e))?;
    Ok(theme_config.current_theme)
}

/// Saves the current theme name for the TUI to current_theme.toml
pub fn save_current_theme(theme_name: &str) -> Result<(), RextTuiError> {
    let theme_config = CurrentTheme {
        current_theme: theme_name.to_string(),
    };
    let contents = toml::to_string(&theme_config).map_err(|e| RextTuiError::SerializeError(e))?;
    fs::write("config/current_theme.toml", contents)
        .map_err(|e| RextTuiError::WriteConfigFile(e))?;
    Ok(())
}

/// Loads the color values for the TUI from rext_tui.toml
pub fn load_theme_colors(theme_name: &str) -> Result<Colors, RextTuiError> {
    let config = load_config()?;

    config
        .themes
        .get(theme_name)
        .cloned()
        .ok_or_else(|| RextTuiError::ThemeNotFound(theme_name.to_string()))
}

/// Gets the available themes for the TUI from rext_tui.toml
pub fn get_available_themes() -> Result<Vec<String>, RextTuiError> {
    let config = load_config()?;
    let mut themes: Vec<String> = config.themes.keys().cloned().collect();
    themes.sort();
    Ok(themes)
}

/// Gets the current language for the TUI from current_localization.toml
pub fn load_current_language() -> Result<String, RextTuiError> {
    let contents = fs::read_to_string(CURRENT_LOCALIZATION_PATH)
        .map_err(|e| RextTuiError::ReadConfigFile(e))?;
    let localization_config: LocalizationConfig =
        toml::from_str(&contents).map_err(|e| RextTuiError::ConfigError(e))?;
    Ok(localization_config.language)
}

/// Saves the current language for the TUI to current_localization.toml
pub fn save_current_language(language: &str) -> Result<(), RextTuiError> {
    let localization_config = CurrentLocalization {
        current_localization: language.to_string(),
    };
    let contents =
        toml::to_string(&localization_config).map_err(|e| RextTuiError::SerializeError(e))?;
    fs::write(CURRENT_LOCALIZATION_PATH, contents).map_err(|e| RextTuiError::WriteConfigFile(e))?;
    Ok(())
}

/// Gets the available languages for the TUI from rext_tui.toml
pub fn get_available_languages() -> Result<Vec<String>, RextTuiError> {
    let config = load_config()?;
    let mut languages: Vec<String> = config.localization.keys().cloned().collect();
    languages.sort();
    Ok(languages)
}
