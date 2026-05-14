use crate::{
    client::WhmcsClient,
    error::WhmcsError,
    models::clients::{
        GetClientDetailsParams, GetClientDetailsResponse, GetClientGroupsResponse, GetClientParams,
        GetClientPasswordParams, GetClientPasswordResponse, GetClientsResponse,
    },
    resources::endpoint,
};

impl WhmcsClient {
    endpoint!(
        /// Obtain a vector of client groups
        ///
        /// # Example
        /// ```no_run
        /// # use whmcs::{WhmcsBuilder, models::clients::GetClientGroupsResponse};
        /// # async fn run() -> Result<(), whmcs::WhmcsError> {
        /// let client = WhmcsBuilder::new()
        ///     .url("https://example.com/includes/api.php")
        ///     .api_identifier("id")
        ///     .api_secret("secret")
        ///     .build()
        ///     .expect("Failed to build client");
        ///
        /// let response = client.get_client_groups().await;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Reference
        /// <https://developers.whmcs.com/api-reference/getclientgroups/>
        ///
        /// # Errors
        /// Will return a [`WhmcsError`](`crate::WhmcsError`) if the API returns an error.
        get_client_groups,
        "GetClientGroups",
        GetClientGroupsResponse
    );
    endpoint!(
        #[deprecated(since = "0.1.0", note = "Deprecated in WHMCS 8.0.0")]
        /// Obtain the encrypted client password
        ///
        /// # Example
        /// ```no_run
        /// # use whmcs::{WhmcsBuilder, models::clients::GetClientPasswordParams};
        /// # async fn run() -> Result<(), whmcs::WhmcsError> {
        /// let client = WhmcsBuilder::new()
        ///     .url("https://example.com/includes/api.php")
        ///     .api_identifier("id")
        ///     .api_secret("secret")
        ///     .build()
        ///     .expect("Failed to build client");
        ///
        /// let response = client.get_client_password(GetClientPasswordParams::default().client_id(1)).await;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Reference
        /// <https://developers.whmcs.com/api-reference/getclientpassword/>
        ///
        /// # Errors
        /// Will return a [`WhmcsError`](`crate::WhmcsError`) if the API returns an error.
        get_client_password,
        "GetClientPassword",
        GetClientPasswordParams,
        GetClientPasswordResponse
    );
    endpoint!(
        /// Obtain the Clients that match passed criteria
        ///
        /// # Examples
        /// ## Getting 25 clients
        /// ```no_run
        /// # use whmcs::{WhmcsBuilder, models::clients::GetClientParams};
        /// # async fn run() -> Result<(), whmcs::WhmcsError> {
        /// let client = WhmcsBuilder::new()
        ///     .url("https://example.com/includes/api.php")
        ///     .api_identifier("id")
        ///     .api_secret("secret")
        ///     .build()
        ///     .expect("Failed to build client");
        ///
        /// let response = client.get_clients(None).await;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// ## Getting clients via a search term
        /// ```no_run
        /// # use whmcs::{WhmcsBuilder, models::clients::GetClientParams};
        /// # async fn run() -> Result<(), whmcs::WhmcsError> {
        /// let client = WhmcsBuilder::new()
        ///     .url("https://example.com/includes/api.php")
        ///     .api_identifier("id")
        ///     .api_secret("secret")
        ///     .build()
        ///     .expect("Failed to build client");
        ///
        /// let response = client.get_clients(GetClientParams::default().search("user@example.com")).await;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// ## Filtering options
        /// ```no_run
        /// # use whmcs::{WhmcsSorting, WhmcsBuilder, models::clients::{GetClientParams, ClientStatus, ClientOrderBy}};
        /// # async fn run() -> Result<(), whmcs::WhmcsError> {
        /// let client = WhmcsBuilder::new()
        ///     .url("https://example.com/includes/api.php")
        ///     .api_identifier("id")
        ///     .api_secret("secret")
        ///     .build()
        ///     .expect("Failed to build client");
        ///
        /// let response = client.get_clients(GetClientParams::default().search("user@example.com").limit_start(10).limit_num(25).sorting(WhmcsSorting::Descending).status(ClientStatus::Active).order_by(ClientOrderBy::Email)).await;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Reference
        /// <https://developers.whmcs.com/api-reference/getclients/>
        ///
        /// # Errors
        /// Will return a [`WhmcsError`](`crate::WhmcsError`) if the API returns an error.
        get_clients,
        "GetClients",
        GetClientParams,
        GetClientsResponse
    );
    endpoint!(
        /// Obtain the Clients Details for a specific client
        ///
        /// # Examples
        /// ## Getting client details
        /// ```no_run
        /// # use whmcs::{WhmcsBuilder, models::clients::GetClientDetailsParams};
        /// # async fn run() -> Result<(), whmcs::WhmcsError> {
        /// let client = WhmcsBuilder::new()
        ///     .url("https://example.com/includes/api.php")
        ///     .api_identifier("id")
        ///     .api_secret("secret")
        ///     .build()
        ///     .expect("Failed to build client");
        ///
        /// let response = client.get_client_details(GetClientDetailsParams::default().email("user@example.com")).await;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// ## Getting client details and statistics
        /// ```no_run
        /// # use whmcs::{WhmcsBuilder, models::clients::GetClientDetailsParams};
        /// # async fn run() -> Result<(), whmcs::WhmcsError> {
        /// let client = WhmcsBuilder::new()
        ///     .url("https://example.com/includes/api.php")
        ///     .api_identifier("id")
        ///     .api_secret("secret")
        ///     .build()
        ///     .expect("Failed to build client");
        ///
        /// let response = client.get_client_details(GetClientDetailsParams::default().client_id(1).include_stats(true)).await;
        /// if let Some(stats) = response.unwrap().stats {
        ///     // ... do something with the stats
        /// }
        /// # Ok(())
        /// # }
        /// ```
        ///
        ///
        /// # Reference
        /// <https://developers.whmcs.com/api-reference/getclientsdetails/>
        ///
        /// # Errors
        /// Will return a [`WhmcsError`](`crate::WhmcsError`) if the API returns an error.
        get_client_details,
        "GetClientsDetails",
        GetClientDetailsParams,
        GetClientDetailsResponse
    );
}

#[cfg(test)]
mod tests {
    use crate::{
        error::WhmcsError,
        models::clients::{
            ClientGroupId, ClientStatus, GetClientDetailsParams, GetClientParams,
            GetClientPasswordParams,
        },
        test::get_test_client,
    };

    #[ignore = "Require WHMCS instance"]
    #[tokio::test]
    async fn get_client_groups() {
        let client = get_test_client();
        let response = client.get_client_groups().await.unwrap();
        assert_eq!(response.total_results, 0);
        assert_eq!(response.groups.len(), 0);
    }

    #[ignore = "Require WHMCS instance"]
    #[tokio::test]
    async fn invalid_get_client_password() {
        let client = get_test_client();
        #[allow(deprecated)]
        let err = client
            .get_client_password(None)
            .await
            .expect_err("should be an error");
        assert!(matches!(err, WhmcsError::ApiError(_)));
    }

    #[ignore = "Require WHMCS instance"]
    #[tokio::test]
    async fn get_client_password() {
        let client = get_test_client();
        #[allow(deprecated)]
        let response = client
            .get_client_password(GetClientPasswordParams::default().client_id(1))
            .await
            .unwrap();
        assert!(response.password.starts_with("$2y$"));
    }

    #[ignore = "Require WHMCS instance"]
    #[tokio::test]
    async fn get_clients() {
        let client = get_test_client();
        let response = client.get_clients(None).await.unwrap();
        assert_ne!(response.total_results, 0);
        assert_eq!(response.number_returned, 25);
        assert_eq!(response.start_number, 0);
        assert_eq!(response.clients.len(), 25);
        assert_eq!(response.clients[0].group_id, ClientGroupId::new(0));
        assert_eq!(response.clients[0].status, ClientStatus::Inactive);
    }

    #[ignore = "Require WHMCS instance"]
    #[tokio::test]
    async fn get_no_clients() {
        let client = get_test_client();
        let response = client
            .get_clients(GetClientParams::default().search("nonexistentuser"))
            .await
            .unwrap();
        assert_eq!(response.total_results, 0);
        assert_eq!(response.number_returned, 0);
        assert_eq!(response.start_number, 0);
        assert_eq!(response.clients.len(), 0);
    }

    #[ignore = "Require WHMCS instance"]
    #[tokio::test]
    async fn get_single_client() {
        let client = get_test_client();
        let email = std::env::var("EMAIL").unwrap();
        let response = client
            .get_clients(GetClientParams::default().search(email.clone()))
            .await
            .unwrap();
        assert_ne!(response.total_results, 0);
        assert_eq!(response.number_returned, 1);
        assert_eq!(response.start_number, 0);
        assert_eq!(response.clients.len(), 1);
        assert_eq!(response.clients[0].group_id, ClientGroupId::new(0));
        assert_eq!(response.clients[0].status, ClientStatus::Active);
        assert_eq!(response.clients[0].email, email);
    }

    #[ignore = "Require WHMCS instance"]
    #[tokio::test]
    async fn get_client_details() {
        let client = get_test_client();
        let email = std::env::var("EMAIL").unwrap();
        let response = client
            .get_client_details(GetClientDetailsParams::default().email(email.clone()))
            .await
            .unwrap();

        assert_eq!(response.client.email, email);
        assert_eq!(response.client.users.len(), 1);
        assert_eq!(response.client.users[0].email, email);
        assert!(response.client.users[0].is_owner);
        assert!(response.stats.is_none());
    }
}
