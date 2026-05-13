use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("WHMCS_URL is not set")]
    UrlNotSet,
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error("WHMCS_API_IDENTIFIER is not set")]
    ApiIdentifierNotSet,
    #[error("WHMCS_API_SECRET is not set")]
    ApiSecretNotSet,
}

#[derive(Debug, Error)]
pub enum WhmcsError {
    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
    #[error(transparent)]
    BuilderError(#[from] BuilderError),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error("{0}")]
    ApiError(String),
}
