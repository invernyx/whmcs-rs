use serde::{Deserialize, Serialize};

use crate::models::clients::ClientId;

#[derive(Debug, Serialize, Default)]
/// Parameters for obtaining the password for a client on [`get_client_password`](crate::WhmcsClient::get_client_password).
pub struct GetClientPasswordParams {
    /// The client ID to obtain the password for
    // WHMCS uses `userid` for the client ID, but we use `client_id` for consistency.
    #[serde(rename = "userid", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    /// The email address to obtain the password for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl GetClientPasswordParams {
    /// The userid to obtain the password for
    #[must_use]
    pub fn client_id(mut self, client_id: impl Into<ClientId>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// The email address to obtain the password for
    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
/// Response from [`get_client_password`](crate::WhmcsClient::get_client_password).
pub struct GetClientPasswordResponse {
    /// The encrypted password for the client
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_client_password_params_serializes_userid() {
        let p = GetClientPasswordParams::default()
            .client_id(ClientId::new(5))
            .email("x@y.z");
        let v = serde_json::to_value(&p).unwrap();
        assert_eq!(v["userid"], 5);
        assert_eq!(v["email"], "x@y.z");
        assert!(!v.as_object().unwrap().contains_key("client_id"));
    }

    #[test]
    fn get_client_password_response_deserializes() {
        let r: GetClientPasswordResponse =
            serde_json::from_str(r#"{"password":"$2y$10$abc"}"#).unwrap();
        assert_eq!(r.password, "$2y$10$abc");
    }
}
