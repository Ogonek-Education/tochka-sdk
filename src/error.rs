use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration Error: {0}")]
    Config(String),
    #[error("Token misses permissions: {0}")]
    MissingScope(String),
    #[error("HTTP Error: {0}")]
    Http(String),
    #[error("API Error: {0}")]
    Api(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        // Strip the error of the token
        Self::Http(err.without_url().to_string())
    }
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::Config(err.to_string())
    }
}
