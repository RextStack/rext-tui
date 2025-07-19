/// Custom error codes for RextTui
#[derive(thiserror::Error, Debug)]
pub enum RextTuiError {
    #[error("Failed to read crossterm event: {0}")]
    ReadEvent(#[from] std::io::Error),
    #[error("Failed to load config: {0}")]
    ConfigError(#[from] toml::de::Error),
    #[error("Failed to read config file: {0}")]
    ReadConfigFile(std::io::Error),
    #[error("Failed to write config file: {0}")]
    WriteConfigFile(std::io::Error),
    #[error("Failed to serialize config: {0}")]
    SerializeError(#[from] toml::ser::Error),
    #[error("Theme '{0}' not found")]
    ThemeNotFound(String),
}
