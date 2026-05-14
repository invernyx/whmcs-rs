use std::time::Duration;

use reqwest::header::CONTENT_TYPE;
use serde_json::Map;
use url::Url;

use crate::{error::WhmcsError, models::WhmcsRawResponse};

/// HTTP client for the WHMCS remote API.
///
/// Construct one with [`WhmcsBuilder::build`](crate::WhmcsBuilder::build) when the **`builder`**
/// feature is enabled (default), or with [`WhmcsClient::new`] if you manage URL and credentials
/// yourself.
///
/// # Requests
///
/// Every call goes through [`WhmcsClient::request`]: parameters are serialized to a JSON object,
/// posted as `application/x-www-form-urlencoded`, and the JSON response is parsed. WHMCS errors
/// become [`WhmcsError::ApiError`](crate::WhmcsError::ApiError).
///
/// Convenience methods (for example [`get_clients`](WhmcsClient::get_clients)) are defined in
/// this crate’s `resources` modules; they delegate to [`request`](WhmcsClient::request).
///
/// # Examples
///
/// ```no_run
/// use whmcs::{WhmcsClient, WhmcsError};
/// use whmcs::models::clients::{GetClientParams, GetClientsResponse};
///
/// # async fn demo(client: WhmcsClient) -> Result<(), WhmcsError> {
/// let found: GetClientsResponse = client
///     .get_clients(GetClientParams::default().search("admin@"))
///     .await?;
/// # let _ = found;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct WhmcsClient {
    client: reqwest::Client,
    url: Url,
    api_identifier: String,
    api_secret: String,
    timeout: u64,
}

impl WhmcsClient {
    /// Create a new WHMCS API client.
    ///
    /// # Parameters
    ///
    /// - `url`: The URL of the WHMCS API.
    /// - `api_identifier`: The API identifier for the WHMCS API.
    /// - `api_secret`: The API secret for the WHMCS API.
    /// - `timeout`: The timeout for the WHMCS API requests.
    #[must_use]
    pub fn new(url: Url, api_identifier: String, api_secret: String, timeout: u64) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
            api_identifier,
            api_secret,
            timeout,
        }
    }

    /// Send a request to the WHMCS API
    ///
    /// This function takes an action (from the WHMCS API Index), the parameters (which must be serializable to JSON)
    /// and the expected response type (which must be deserializable from JSON).
    ///
    /// It returns a `Result` containing the deserialized response or a `WhmcsError` if the request fails.
    ///
    /// # Errors
    ///
    /// - `WhmcsError::SerializationError` if the parameters cannot be serialized to JSON
    /// - `WhmcsError::RequestError` if the request fails
    /// - `WhmcsError::ApiError` if the API returns an error
    pub async fn request<P, T>(&self, action: &str, params: P) -> Result<T, WhmcsError>
    where
        P: serde::Serialize,
        T: serde::de::DeserializeOwned,
    {
        let mut request_body =
            serde_json::to_value(params).map_err(WhmcsError::SerializationError)?;

        if request_body.is_null() {
            request_body = serde_json::Value::Object(Map::new());
        }
        if let Some(obj) = request_body.as_object_mut() {
            obj.insert("action".to_string(), action.into());
            obj.insert("identifier".to_string(), self.api_identifier.clone().into());
            obj.insert("secret".to_string(), self.api_secret.clone().into());
            obj.insert("responsetype".to_string(), "json".into());
        }

        let response_text = self
            .client
            .post(self.url.as_str())
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&request_body)
            .timeout(Duration::from_secs(self.timeout))
            .send()
            .await
            .map_err(WhmcsError::RequestError)?
            .text()
            .await
            .map_err(WhmcsError::RequestError)?;

        match serde_json::from_str::<WhmcsRawResponse<T>>(&response_text)
            .map_err(WhmcsError::SerializationError)?
        {
            WhmcsRawResponse::Success(data) => Ok(data),
            WhmcsRawResponse::Error { message } => Err(WhmcsError::ApiError(message)),
        }
    }
}
