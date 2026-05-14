# whmcs

Rust client for the [WHMCS API](https://developers.whmcs.com/api/) (`includes/api.php`). POSTs form-encoded JSON-shaped fields, parses, and turns `result: success | error` into `Result<T, WhmcsError>`.

## Installation

```toml
[dependencies]
whmcs = "0.1.0"
```

## Create a client

```rust
use whmcs::::WhmcsBuilder;
use whmcs::WhmcsError;

async fn example() -> Result<(), WhmcsError> {
    let client = WhmcsBuilder::new()
        .url("https://billing.example.com/includes/api.php")
        .api_identifier("...")
        .api_secret("...")
        .timeout(30) // seconds; optional, default 30
        .build()?;

    // call API methods on `client`
    Ok(())
}
```

**URL:** If the path does not end with `api.php`, the builder adjusts it so requests hit `.../includes/api.php`.

**Environment:** Any of `.url`, `.api_identifier`, `.api_secret` can be omitted and filled from env at `build()` time:

| If unset | `build()` uses |
|----------------|----------------|
| URL | `WHMCS_URL` |
| Identifier | `WHMCS_API_IDENTIFIER` |
| Secret | `WHMCS_API_SECRET` |
| Timeout | `WHMCS_TIMEOUT` (seconds), else `30` |

## Call the API

High-level methods live on `WhmcsClient` (see `whmcs::resources` via the same client type). Request types and response structs are under `whmcs::models`.

**Parameters:** Endpoints that take params accept `None` (WHMCS defaults / empty body for that action) or a params value (wrapped as `Some` automatically on recent Rust).

```rust
use whmcs::models::auth::ValidateLoginParams;
use whmcs::models::clients::{GetClientDetailsParams, GetClientParams, GetClientsResponse};

// No extra filters (WHMCS defaults)
let all: GetClientsResponse = client.get_clients(None).await?;

// Filtered list
let filtered = client
    .get_clients(GetClientParams::default().search("user@example.com"))
    .await?;

// Client details by email
let details = client
    .get_client_details(GetClientDetailsParams::default().email("user@example.com"))
    .await?;

// ValidateLogin
let session = client
    .validate_login(
        ValidateLoginParams::default()
            .email("user@example.com")
            .password("..."),
    )
    .await?;
```

**Actions without a wrapper yet:** use the public generic:

```rust
client.request("YourAction", your_params_struct).await?;
```

`your_params_struct` must serialize to a JSON **object** (key/value pairs). The success payload (everything after the `result` discriminator) must deserialize into `T`.

## Errors

| Type | When |
|------|------|
| `BuilderError` | `build()` failed: missing URL or credentials, bad URL |
| `WhmcsError::ApiError(msg)` | WHMCS returned `result: error` |
| `WhmcsError::RequestError` | Network / HTTP |
| `WhmcsError::SerializationError` | JSON encode/decode mismatch |

## Implemented methods

| `WhmcsClient` method | WHMCS action |
|----------------------|--------------|
| `validate_login` | `ValidateLogin` |
| `get_client_groups` | `GetClientGroups` |
| `get_client_password` | `GetClientPassword` |
| `get_clients` | `GetClients` |
| `get_client_details` | `GetClientsDetails` |

Field names and behaviour match the official API reference for each action.

---

## License

Licensed under the [MIT License](LICENSE). Copyright (c) 2026 TFDI Design Co.
