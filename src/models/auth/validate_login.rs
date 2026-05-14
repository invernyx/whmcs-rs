use serde::{Deserialize, Serialize};

use crate::models::{deserialize_whmcs_bool, users::UserId};

#[derive(Debug, Serialize, Default)]
/// Parameters for validating a login on [`validate_login`](crate::WhmcsClient::validate_login).
pub struct ValidateLoginParams {
    /// User Email Address
    pub email: String,
    /// Password to validate
    #[serde(rename = "password2")]
    pub password: String,
}

impl ValidateLoginParams {
    /// User Email Address
    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    /// Password to validate
    #[must_use]
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
/// Response from [`validate_login`](crate::WhmcsClient::validate_login).
pub struct ValidateLoginResponse {
    /// User ID
    #[serde(rename = "userid")]
    pub user_id: UserId,
    /// Login session token - returned if Two-Factor Authentication is not required for the account
    #[serde(rename = "passwordhash")]
    pub session_token: Option<String>,
    /// True if Two-Factor Authentication is enabled for the given account
    #[serde(
        rename = "twoFactorEnabled",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub two_factor_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_login_params_serializes_password2() {
        let p = ValidateLoginParams::default()
            .email("a@example.com")
            .password("secret");
        let v = serde_json::to_value(&p).unwrap();
        assert_eq!(v["email"], "a@example.com");
        assert_eq!(v["password2"], "secret");
        assert!(!v.as_object().unwrap().contains_key("password"));
    }

    #[test]
    fn validate_login_response_deserializes() {
        let j = r#"{"userid":7,"passwordhash":"tok","twoFactorEnabled":"1"}"#;
        let r: ValidateLoginResponse = serde_json::from_str(j).unwrap();
        assert_eq!(r.user_id.as_u32(), 7);
        assert_eq!(r.session_token.as_deref(), Some("tok"));
        assert!(r.two_factor_enabled);
    }
}
