#[warn(missing_docs)]
#[warn(clippy::pedantic)]
#[warn(clippy::cargo)]
#[warn(clippy::nursery)]
/// The builder module contains the `WhmcsBuilder` struct, which is used to build a `WhmcsClient` instance. This is how all interactions with the WHMCS API are made.
pub mod builder;
pub mod error;
pub mod models;
pub mod resources;
#[cfg(test)]
pub mod test;
