pub type Result<T> = std::result::Result<T, MCLError>;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MCLError {
    #[error("HTTP request failed: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Filesystem utility error: {0}")]
    FsutilError(#[from] crate::fsutil::IOError),

    #[error("HTTP error: {0}")]
    HttpError(#[from] crate::http::HttpError),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}