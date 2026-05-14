use thiserror::Error;

#[derive(Debug, Error)]
#[cfg(feature = "builder")]
/// Errors that can occur when building a `WhmcsClient`.
pub enum BuilderError {
    #[error("WHMCS_URL is not set")]
    /// The URL is not set.
    UrlNotSet,
    #[error(transparent)]
    /// The URL is not valid.
    UrlParseError(#[from] url::ParseError),
    #[error("WHMCS_API_IDENTIFIER is not set")]
    /// The API identifier is not set.
    ApiIdentifierNotSet,
    #[error("WHMCS_API_SECRET is not set")]
    /// The API secret is not set.
    ApiSecretNotSet,
}

#[derive(Debug, Error)]
/// Errors that can occur when making a request to the WHMCS API.
pub enum WhmcsError {
    #[error(transparent)]
    /// The parameters cannot be serialized to JSON.
    SerializationError(#[from] serde_json::Error),
    #[cfg(feature = "builder")]
    #[error(transparent)]
    /// An error occurred when building the client.
    BuilderError(#[from] BuilderError),
    #[error(transparent)]
    /// An error occurred when making the request.
    RequestError(#[from] reqwest::Error),
    #[error("{0}")]
    /// An error occurred when the API returned an error.
    ApiError(String),
}
