/// Custom error codes for RextTui
#[derive(thiserror::Error, Debug)]
pub enum RextTuiError {
    #[error("Failed to read crossterm event: {0}")]
    ReadEvent(#[from] std::io::Error),
}
