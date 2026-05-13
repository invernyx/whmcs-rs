use crate::builder::{WhmcsBuilder, WhmcsClient};

pub fn get_test_client() -> WhmcsClient {
    dotenvy::dotenv().ok();

    let url = std::env::var("WHMCS_URL").expect("WHMCS_URL must be set");
    let id = std::env::var("WHMCS_IDENTIFIER").expect("WHMCS_IDENTIFIER must be set");
    let secret = std::env::var("WHMCS_SECRET").expect("WHMCS_SECRET must be set");

    WhmcsBuilder::new()
        .url(url)
        .api_identifier(id)
        .api_secret(secret)
        .build()
        .expect("Failed to build test client")
}
