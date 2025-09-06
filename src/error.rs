use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("asset not found")]
    NotFound,

    #[error("unsupported format")]
    UnsupportedFormat,

    #[error("decode error: {0}")]
    DecodeError(String),

    #[error("io error: {0}")]
    IoError(String),
}