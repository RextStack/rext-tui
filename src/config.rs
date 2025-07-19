use serde::Deserialize;
use std::fs;

use crate::error::RextTuiError;

#[derive(Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Deserialize)]
pub struct Config {
    pub default_colors: Colors,
    pub dracula_theme: Colors,
}

#[derive(Deserialize)]
pub struct Colors {
    pub primary: Rgb,
    pub text: Rgb,
    pub background: Rgb,
}

pub fn load_config(path: &str) -> Result<Config, RextTuiError> {
    let contents = fs::read_to_string(path).map_err(|e| RextTuiError::ReadConfigFile(e))?;
    let config: Config = toml::from_str(&contents).map_err(|e| RextTuiError::ConfigError(e))?;
    Ok(config)
}
