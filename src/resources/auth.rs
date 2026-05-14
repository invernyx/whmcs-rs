use crate::{
    client::WhmcsClient,
    error::WhmcsError,
    models::auth::{ValidateLoginParams, ValidateLoginResponse},
    resources::endpoint,
};

impl WhmcsClient {
    endpoint!(
        /// This command can be used to validate an email address and password against a registered user in WHMCS.
        ///
        /// If the email address is valid, and the password is correct, a session will be started as part of creating a passwordhash. This session can be used for remote sign-in, (see below) but can affect other aspects of your script if using the `LocalAPI`. A validated login will update the last login time stored for the client account. On success, the `user_id` and `session_token` will be returned. This can be used to establish an authenticated session by setting the session key `‘login_auth_tk’` to the `session_token`.
        ///
        /// **Note:** If session IP validation is enabled, this API call must be executed via the local API to receive a valid hash.
        ///
        /// **Note:** The login functionality provided by this API is superseded by `CreateSsoToken` <https://developers.whmcs.com/api-reference/createssotoken/> for remote login functionality and `OpenID Connect` <https://go.whmcs.com/1993/openid-connect> for authentication services/credential verification. This API may be deprecated in the future given these more robust and modern implementations.
        ///
        /// # Example
        /// ```no_run
        /// # use whmcs::{WhmcsBuilder, models::auth::ValidateLoginParams};
        /// # async fn run() -> Result<(), whmcs::WhmcsError> {
        /// let client = WhmcsBuilder::new()
        ///     .url("https://example.com/includes/api.php")
        ///     .api_identifier("id")
        ///     .api_secret("secret")
        ///     .build()
        ///     .expect("Failed to build client");
        ///
        /// let response = client
        ///     .validate_login(ValidateLoginParams::default()
        ///         .email("user@example.com")
        ///         .password("password"))
        ///     .await;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Reference
        /// <https://developers.whmcs.com/api-reference/validatelogin/>
        ///
        /// # Errors
        /// Will return a [`WhmcsError`](`crate::WhmcsError`) if the API returns an error.
        validate_login,
        "ValidateLogin",
        ValidateLoginParams,
        ValidateLoginResponse
    );
}

#[cfg(test)]
mod tests {
    use crate::{error::WhmcsError, models::auth::ValidateLoginParams, test::get_test_client};

    fn test_password() -> String {
        std::env::var("PASSWORD").expect("set PASSWORD for validate_login integration tests")
    }

    #[ignore]
    #[tokio::test]
    async fn validate_login_success() {
        let client = get_test_client();
        let email = std::env::var("EMAIL")
            .expect("set EMAIL to a WHMCS account email for validate_login integration tests");
        let password = test_password();

        let response = client
            .validate_login(
                ValidateLoginParams::default()
                    .email(email)
                    .password(password),
            )
            .await
            .expect("validate_login should succeed with correct credentials");

        assert_ne!(response.user_id.as_u32(), 0);
    }

    #[ignore]
    #[tokio::test]
    async fn validate_login_wrong_password() {
        let client = get_test_client();
        let email = std::env::var("EMAIL")
            .expect("set EMAIL to a WHMCS account email for validate_login integration tests");

        let err = client
            .validate_login(
                ValidateLoginParams::default()
                    .email(email)
                    .password("definitely-not-the-real-password-9f3a2c1e"),
            )
            .await
            .expect_err("wrong password should yield an API error");

        assert!(matches!(err, WhmcsError::ApiError(_)));
    }
}
