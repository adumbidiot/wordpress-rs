use std::result::Result as StdResult;

/// Result Type
///
pub type Result<T> = StdResult<T, Error>;

/// Error Type
///
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reqwest HTTP Error
    ///
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    /// A path fragment was invalid
    ///
    #[error("invalid path fragment ({0})")]
    InvalidPathFragment(url::ParseError),

    /// Invalid HTTP Status
    ///
    #[error("{0}")]
    InvalidStatus(reqwest::StatusCode),

    /// Json Error
    ///
    #[error("{0}")]
    Json(#[from] serde_json::Error),

    /// Query String encode failed
    #[error("{0}")]
    QueryEncode(serde_urlencoded::ser::Error),

    /// Missing total header
    ///
    #[error("missing total header")]
    MissingTotal,

    /// Io Error
    ///
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
