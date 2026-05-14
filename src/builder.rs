use url::Url;

use crate::{client::WhmcsClient, error::BuilderError};

/// A builder for the WHMCS API client.
#[derive(Debug)]
pub struct WhmcsBuilder {
    url: Option<String>,
    api_identifier: Option<String>,
    api_secret: Option<String>,
    timeout: Option<u64>,
}

impl Default for WhmcsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "builder")]
impl WhmcsBuilder {
    /// Create a new builder for the WHMCS API client.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            url: None,
            api_identifier: None,
            api_secret: None,
            timeout: None,
        }
    }

    /// Set the URL of the WHMCS API.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the API identifier for the WHMCS API.
    #[must_use]
    pub fn api_identifier(mut self, api_identifier: impl Into<String>) -> Self {
        self.api_identifier = Some(api_identifier.into());
        self
    }

    /// Set the API secret for the WHMCS API.
    #[must_use]
    pub fn api_secret(mut self, api_secret: impl Into<String>) -> Self {
        self.api_secret = Some(api_secret.into());
        self
    }

    /// Set the timeout for the WHMCS API requests (default 30 seconds)
    #[must_use]
    pub fn timeout(mut self, timeout: impl Into<u64>) -> Self {
        self.timeout = Some(timeout.into());
        self
    }

    /// Build the WHMCS API client.
    ///
    /// # Errors
    ///
    /// - `BuilderError::UrlNotSet` if the URL is not set
    /// - `BuilderError::UrlParseError` if the URL is invalid
    /// - `BuilderError::ApiIdentifierNotSet` if the API identifier is not set
    /// - `BuilderError::ApiSecretNotSet` if the API secret is not set
    pub fn build(self) -> Result<WhmcsClient, BuilderError> {
        let input_url = self
            .url
            .or_else(|| std::env::var("WHMCS_URL").ok())
            .ok_or(BuilderError::UrlNotSet)?
            .parse::<Url>()
            .map_err(BuilderError::UrlParseError)?;
        let mut url = input_url.clone();
        if let Some(mut path_segments) = input_url.path_segments() {
            let endpoint = path_segments.next_back().unwrap_or_default();
            if endpoint.is_empty() {
                url.set_path("/includes/api.php");
            } else if endpoint != "api.php" {
                url.set_path(&format!("{}/includes/api.php", input_url.path()));
            }
        }

        let api_identifier = self
            .api_identifier
            .or_else(|| std::env::var("WHMCS_API_IDENTIFIER").ok())
            .ok_or(BuilderError::ApiIdentifierNotSet)?;
        let api_secret = self
            .api_secret
            .or_else(|| std::env::var("WHMCS_API_SECRET").ok())
            .ok_or(BuilderError::ApiSecretNotSet)?;
        let timeout = self.timeout.unwrap_or_else(|| {
            std::env::var("WHMCS_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30)
        });

        Ok(WhmcsClient::new(url, api_identifier, api_secret, timeout))
    }
}

#[cfg(test)]
mod tests {
    use temp_env::with_vars;

    use super::{WhmcsBuilder, WhmcsClient};
    use crate::error::BuilderError;

    #[test]
    fn build_client_with_env_vars() {
        with_vars(
            [
                ("WHMCS_URL", Some("https://example.com/includes/api.php")),
                ("WHMCS_API_IDENTIFIER", Some("id")),
                ("WHMCS_API_SECRET", Some("secret")),
            ],
            || assert!(WhmcsBuilder::new().build().is_ok()),
        );
    }

    #[test]
    fn build_succeeds_with_explicit_credentials() {
        let client: WhmcsClient = WhmcsBuilder::new()
            .url("https://example.com/includes/api.php")
            .api_identifier("id")
            .api_secret("secret")
            .build()
            .unwrap();
        let _ = client;
    }

    #[test]
    fn build_fails_url_not_set_when_env_missing() {
        with_vars(
            [
                ("WHMCS_URL", None::<&str>),
                ("WHMCS_API_IDENTIFIER", None::<&str>),
                ("WHMCS_API_SECRET", None::<&str>),
            ],
            || match WhmcsBuilder::new().build() {
                Err(e) => assert!(matches!(e, BuilderError::UrlNotSet)),
                Ok(_) => panic!("expected build to fail"),
            },
        );
    }

    #[test]
    fn build_fails_api_identifier_not_set() {
        with_vars(
            [("WHMCS_API_IDENTIFIER", None::<&str>)],
            || match WhmcsBuilder::new()
                .url("https://example.com/includes/api.php")
                .api_secret("secret")
                .build()
            {
                Err(e) => assert!(matches!(e, BuilderError::ApiIdentifierNotSet)),
                Ok(_) => panic!("expected build to fail"),
            },
        );
    }

    #[test]
    fn build_fails_api_secret_not_set() {
        with_vars(
            [("WHMCS_API_SECRET", None::<&str>)],
            || match WhmcsBuilder::new()
                .url("https://example.com/includes/api.php")
                .api_identifier("id")
                .build()
            {
                Err(e) => assert!(matches!(e, BuilderError::ApiSecretNotSet)),
                Ok(_) => panic!("expected build to fail"),
            },
        );
    }

    #[test]
    fn build_fails_on_invalid_url() {
        with_vars(
            [
                ("WHMCS_URL", None::<&str>),
                ("WHMCS_API_IDENTIFIER", None::<&str>),
                ("WHMCS_API_SECRET", None::<&str>),
            ],
            || match WhmcsBuilder::new()
                .url(":::not-a-valid-url")
                .api_identifier("id")
                .api_secret("secret")
                .build()
            {
                Err(e) => assert!(matches!(e, BuilderError::UrlParseError(_))),
                Ok(_) => panic!("expected build to fail"),
            },
        );
    }
}
