//! Serde types and helpers for the WHMCS JSON API.
//!
//! This module is organized by WHMCS domain:
//!
//! - [`auth`] — [`ValidateLoginParams`](auth::ValidateLoginParams) and related types.
//! - [`clients`] — client lists, details, groups, and strongly typed IDs such as [`ClientId`](clients::ClientId).
//! - [`users`] — shared [`UserId`](users::UserId) and similar primitives.
//!
//! Cross-cutting pieces live at the root of `models`:
//!
//! - [`WhmcsRawResponse`] — deserializes the `result: success | error` envelope.
//! - [`WhmcsSorting`] — `ASC` / `DESC` for endpoints that support sort direction.
//!
//! Most structs and enums here are shaped to match WHMCS field names via `serde` attributes.
//! When the API returns inconsistent types (booleans as `0`/`1`, strings, or JSON booleans),
//! custom deserializers normalize them for Rust.

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// Models for endpoints that are indexed as `Authentication`.
pub mod auth;
/// Models for endpoints that are indexed as `Clients`.
pub mod clients;
/// Models for endpoints that are indexed as `Users`.
pub mod users;

#[derive(Debug, Deserialize)]
#[serde(tag = "result", rename_all = "lowercase")]
/// The response structure from the WHMCS API. This checks against the `result` field and returns the variant that matches.
pub enum WhmcsRawResponse<T> {
    /// A successful response from the WHMCS API. `T` is the type of the response body.
    Success(T),
    /// An error response from the WHMCS API.
    Error {
        /// The error message returned by the WHMCS API.
        message: String,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
/// The sorting direction for the WHMCS API, used in several endpoints.
pub enum WhmcsSorting {
    /// Sorting in ascending order.
    #[serde(rename = "ASC")]
    Ascending,
    /// Sorting in descending order.
    #[serde(rename = "DESC")]
    Descending,
}

fn deserialize_whmcs_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::Bool(b) => Ok(b),
        Value::Number(n) => Ok(n
            .as_u64()
            .map(|x| x != 0)
            .or_else(|| n.as_i64().map(|x| x != 0))
            .unwrap_or(false)),
        Value::String(s) => Ok(matches!(
            s.as_str(),
            "on" | "1" | "true" | "yes" | "TRUE" | "True"
        )),
        _ => Ok(false),
    }
}

macro_rules! whmcs_nested_vec {
    ($func_name:ident, $inner_key:ident, $inner:ident) => {
        ::paste::paste! {
            fn $func_name<'de, D>(deserializer: D) -> Result<Vec<$inner>, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(serde::Deserialize)]
                struct [<$inner Envelope>] {
                    #[serde(default)]
                    $inner_key: Vec<$inner>,
                }

                let v: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
                if v.is_string() && v.as_str() == Some("") {
                    return Ok(vec![]);
                }

                serde_json::from_value::<[<$inner Envelope>]>(v)
                    .map(|e| e.$inner_key)
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

macro_rules! u32_id {
    ($id_name:ident) => {
        #[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
        #[serde(transparent)]
        #[doc = concat!(stringify!($id_name), " is a wrapper around a u32 ID.")]
        pub struct $id_name(u32);

        impl $id_name {
            #[doc = concat!("Create a new ", stringify!($id_name), " from a u32 ID.")]
            pub const fn new(id: u32) -> Self {
                Self(id)
            }

            #[doc = concat!("Convert the ", stringify!($id_name), " to a u32 ID.")]
            pub const fn as_u32(&self) -> u32 {
                self.0
            }
        }

        impl From<u32> for $id_name {
            fn from(id: u32) -> Self {
                Self::new(id)
            }
        }

        impl From<$id_name> for u32 {
            fn from(id: $id_name) -> Self {
                id.as_u32()
            }
        }
    };
}

macro_rules! mod_export {
    ($mod_name:ident) => {
        mod $mod_name;
        pub use $mod_name::*;
    };
}

use mod_export;
pub(crate) use u32_id;
pub(crate) use whmcs_nested_vec;

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::models::{WhmcsRawResponse, WhmcsSorting, deserialize_whmcs_bool};

    #[derive(Debug, Deserialize)]
    struct BoolField {
        #[serde(deserialize_with = "deserialize_whmcs_bool")]
        flag: bool,
    }

    fn deser_bool(json: &str) -> bool {
        let v: BoolField = serde_json::from_str(json).unwrap();
        v.flag
    }

    #[test]
    fn deserialize_whmcs_bool_from_json_bool() {
        assert!(!deser_bool(r#"{"flag":false}"#));
        assert!(deser_bool(r#"{"flag":true}"#));
    }

    #[test]
    fn deserialize_whmcs_bool_from_string_forms() {
        assert!(deser_bool(r#"{"flag":"1"}"#));
        assert!(deser_bool(r#"{"flag":"on"}"#));
        assert!(deser_bool(r#"{"flag":"true"}"#));
        assert!(!deser_bool(r#"{"flag":"0"}"#));
        assert!(!deser_bool(r#"{"flag":"no"}"#));
    }

    #[test]
    fn deserialize_whmcs_bool_from_number() {
        assert!(deser_bool(r#"{"flag":1}"#));
        assert!(!deser_bool(r#"{"flag":0}"#));
    }

    #[test]
    fn deserialize_whmcs_bool_null_is_false() {
        assert!(!deser_bool(r#"{"flag":null}"#));
    }

    #[test]
    fn whmcs_raw_response_success_and_error() {
        let ok: WhmcsRawResponse<serde_json::Value> =
            serde_json::from_str(r#"{"result":"success","foo":1}"#).unwrap();
        match ok {
            WhmcsRawResponse::Success(v) => assert_eq!(v["foo"], 1),
            WhmcsRawResponse::Error { .. } => panic!("expected success"),
        }

        let err: WhmcsRawResponse<serde_json::Value> =
            serde_json::from_str(r#"{"result":"error","message":"bad"}"#).unwrap();
        match err {
            WhmcsRawResponse::Success(_) => panic!("expected error"),
            WhmcsRawResponse::Error { message } => assert_eq!(message, "bad"),
        }
    }

    #[test]
    fn whmcs_sorting_serializes_uppercase() {
        assert_eq!(
            serde_json::to_string(&WhmcsSorting::Ascending).unwrap(),
            "\"ASC\""
        );
        assert_eq!(
            serde_json::to_string(&WhmcsSorting::Descending).unwrap(),
            "\"DESC\""
        );
    }
}
