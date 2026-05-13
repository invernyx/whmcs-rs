use crate::{
    builder::WhmcsClient,
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
        /// Reference: https://developers.whmcs.com/api-reference/getclientgroups/
        get_client_groups,
        "GetClientGroups",
        GetClientGroupsResponse
    );
    endpoint!(
        #[deprecated(since = "0.1.0", note = "Deprecated in WHMCS 8.0.0")]
        /// Obtain the encrypted client password
        ///
        /// Reference: https://developers.whmcs.com/api-reference/getclientpassword/
        get_client_password,
        "GetClientPassword",
        GetClientPasswordParams,
        GetClientPasswordResponse
    );
    endpoint!(
        /// Obtain the Clients that match passed criteria
        ///
        /// Reference: https://developers.whmcs.com/api-reference/getclients/
        get_clients,
        "GetClients",
        GetClientParams,
        GetClientsResponse
    );
    endpoint!(
        /// Obtain the Clients Details for a specific client
        ///
        /// *Internal usage note:* this function returns the client information in the top level array. This information is deprecated and may be removed in a future version of WHMCS.
        ///
        /// Reference: https://developers.whmcs.com/api-reference/getclientsdetails/
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

    #[ignore]
    #[tokio::test]
    async fn get_client_groups() {
        let client = get_test_client();
        let response = client.get_client_groups().await.unwrap();
        assert_eq!(response.total_results, 0);
        assert_eq!(response.groups.len(), 0);
    }

    #[ignore]
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

    #[ignore]
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

    #[ignore]
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

    #[ignore]
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

    #[ignore]
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

    #[ignore]
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
