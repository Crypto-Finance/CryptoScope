use thiserror::Error;

/// Main error type for CryptoScope
#[derive(Error, Debug)]
pub enum CryptoScopeError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Exchange error: {0}")]
    #[allow(dead_code)]
    ExchangeError(String),

    #[error("Pagination error: {0}")]
    #[allow(dead_code)]
    PaginationError(String),

    #[error("Invalid category: {0}. Must be 'linear', 'inverse', or 'all'")]
    #[allow(dead_code)]
    InvalidCategory(String),

    #[error("Unknown exchange: {0}. Supported: bybit")]
    UnknownExchange(String),

    #[error("API returned error code {code}: {message}")]
    ApiError { code: i32, message: String },
}

pub type Result<T> = std::result::Result<T, CryptoScopeError>;
