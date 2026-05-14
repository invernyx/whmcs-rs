#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(test, deny(warnings))]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![allow(clippy::multiple_crate_versions)]

//! # whmcs
//!
//! Async HTTP client for the [WHMCS][whmcs-api] remote API (`includes/api.php`).
//! It sends `application/x-www-form-urlencoded` bodies, asks WHMCS for JSON
//! (`responsetype=json`), and maps the envelope `result: success | error` to `Result<T, WhmcsError>`
//! ([`WhmcsError`]).
//!
//! Typical use looks like this:
//!
//! - Build a [`WhmcsClient`] (usually with [`WhmcsBuilder`] when the **`builder`** feature is enabled, which is the default).
//! - Call typed helpers such as [`WhmcsClient::get_clients`], or the generic [`WhmcsClient::request`] for any WHMCS action.
//! - Use types from [`models`] for request parameters and deserialized responses.
//!
//! ## Quick start
//!
//! ```no_run
//! use whmcs::{WhmcsBuilder, WhmcsError};
//! use whmcs::models::clients::{GetClientParams, GetClientsResponse};
//!
//! async fn example() -> Result<(), WhmcsError> {
//!     let client = WhmcsBuilder::new()
//!         .url("https://billing.example.com/includes/api.php")
//!         .api_identifier("your-api-identifier")
//!         .api_secret("your-api-secret")
//!         .timeout(30_u64)
//!         .build()?;
//!
//!     let list: GetClientsResponse = client
//!         .get_clients(GetClientParams::default().search("user@example.com"))
//!         .await?;
//!
//!     println!("total matches: {}", list.total_results);
//!     Ok(())
//! }
//! ```
//!
//! ## Building a client
//!
//! With the default **`builder`** feature, use [`WhmcsBuilder`] to set the API URL, credentials,
//! and optional timeout, then call [`WhmcsBuilder::build`]. Any field you omit on the builder is
//! filled from environment variables when `build` runs:
//!
//! | Builder field | Environment variable |
//! |---------------|------------------------|
//! | URL | `WHMCS_URL` |
//! | Identifier | `WHMCS_API_IDENTIFIER` |
//! | Secret | `WHMCS_API_SECRET` |
//! | Timeout (seconds) | `WHMCS_TIMEOUT` (falls back to `30`) |
//!
//! If the URL path does not end with `api.php`, the builder normalizes it so requests target
//! `.../includes/api.php` under your WHMCS base path.
//!
//! With **`default-features = false`** (disabling **`builder`**), construct [`WhmcsClient`] with
//! [`WhmcsClient::new`] and supply URL and secrets yourself.
//!
//! ## Requests and responses
//!
//! - **Typed helpers** — Methods on [`WhmcsClient`] (implemented in this crate’s `resources`
//!   modules) call [`WhmcsClient::request`] with the correct WHMCS `action` name and serde types.
//! - **Generic access** — [`WhmcsClient::request`] accepts any `P` that implements `serde::Serialize`
//!   into a JSON **object** (flat key/value pairs), and any success body type `T` that implements
//!   `serde::de::DeserializeOwned` for the payload after the `result` field is handled by [`WhmcsRawResponse`].
//!
//! WHMCS often encodes booleans and nested lists in non-standard ways; this crate uses custom
//! deserializers in [`models`] where needed.
//!
//! ## Errors
//!
//! See [`WhmcsError`] for request, JSON, and WHMCS-level failures. Configuration problems when
//! using the builder are reported as [`BuilderError`].
//!
//! ## Cargo features
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | **`builder`** | yes | [`WhmcsBuilder`], [`BuilderError`], and `BuilderError` inside [`WhmcsError`]. |
//!
//! ## Further reading
//!
//! - [WHMCS API index][whmcs-api] — official action names, parameters, and behaviour.
//!
//! [whmcs-api]: https://developers.whmcs.com/api/

pub mod models;

#[cfg(feature = "builder")]
mod builder;
mod client;
mod error;
mod resources;
#[cfg(test)]
mod test;

#[cfg(feature = "builder")]
pub use crate::{builder::WhmcsBuilder, error::BuilderError};
pub use client::WhmcsClient;
pub use error::WhmcsError;
pub use models::{WhmcsRawResponse, WhmcsSorting};
