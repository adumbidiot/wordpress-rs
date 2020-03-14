pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    InvalidPathFragment(url::ParseError),
    InvalidStatus(reqwest::StatusCode),
    Json(serde_json::Error),
    QueryEncode(serde_urlencoded::ser::Error),

    MissingTotal,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}
