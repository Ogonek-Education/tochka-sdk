use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration Error: {0}")]
    Config(String),

    #[error("Network Error: {0}")]
    Network(String),

    #[error("Timeout")]
    Timeout,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Not Found")]
    NotFound,

    #[error("Too Many Requests")]
    TooManyRequests,

    #[error("Server Error: {0}")]
    Server(String),

    #[error("Deserialize Error: {0}")]
    Deserialize(String),

    #[error("API Error: {0}")]
    Api(String),

    #[error("Validation Error: {0}")]
    Validation(#[from] validator::ValidationError),
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::Config(err.to_string())
    }
}
