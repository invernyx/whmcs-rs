use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub mod auth;
pub mod clients;
pub mod users;

#[derive(Deserialize)]
#[serde(tag = "result", rename_all = "lowercase")]
pub enum WhmcsRawResponse<T> {
    Success(T),
    Error { message: String },
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum WhmcsSorting {
    #[serde(rename = "ASC")]
    Ascending,
    #[serde(rename = "DESC")]
    Descending,
}

fn deserialize_whmcs_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::Bool(b) => Ok(b),
        Value::Null => Ok(false),
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
        pub struct $id_name(u32);

        impl $id_name {
            pub fn new(id: u32) -> Self {
                Self(id)
            }

            pub fn as_u32(&self) -> u32 {
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
pub(crate) use u32_id;
pub(crate) use whmcs_nested_vec;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct BoolField {
        #[serde(deserialize_with = "super::deserialize_whmcs_bool")]
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
