use serde::{Deserialize, Serialize};

use crate::models::{deserialize_whmcs_bool, users::UserId};

#[derive(Debug, Serialize, Default)]
pub struct ValidateLoginParams {
    /// User Email Address
    pub email: String,
    /// Password to validate
    #[serde(rename = "password2")]
    pub password: String,
}

impl ValidateLoginParams {
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
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
            .email("u@example.com")
            .password("secret");
        let v = serde_json::to_value(&p).unwrap();
        assert_eq!(v["email"], "u@example.com");
        assert_eq!(v["password2"], "secret");
    }

    #[test]
    fn validate_login_response_deserializes() {
        let json = r#"{
            "userid": 99,
            "passwordhash": "abc",
            "twoFactorEnabled": false
        }"#;
        let r: ValidateLoginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.user_id, UserId::new(99));
        assert_eq!(r.session_token.as_deref(), Some("abc"));
        assert!(!r.two_factor_enabled);
    }
}
