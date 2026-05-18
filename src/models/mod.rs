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
/// Models for endpoints that are indexed as `System`.
pub mod system;
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

#[derive(Debug, Deserialize, Default, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
/// Represents a country.
pub enum Country {
    #[serde(rename = "AF")]
    Afghanistan,
    #[serde(rename = "AX")]
    AlandIslands,
    #[serde(rename = "AL")]
    Albania,
    #[serde(rename = "DZ")]
    Algeria,
    #[serde(rename = "AS")]
    AmericanSamoa,
    #[serde(rename = "AD")]
    Andorra,
    #[serde(rename = "AO")]
    Angola,
    #[serde(rename = "AI")]
    Anguilla,
    #[serde(rename = "AQ")]
    Antarctica,
    #[serde(rename = "AG")]
    AntiguaAndBarbuda,
    #[serde(rename = "AR")]
    Argentina,
    #[serde(rename = "AM")]
    Armenia,
    #[serde(rename = "AW")]
    Aruba,
    #[serde(rename = "AU")]
    Australia,
    #[serde(rename = "AT")]
    Austria,
    #[serde(rename = "AZ")]
    Azerbaijan,
    #[serde(rename = "BS")]
    Bahamas,
    #[serde(rename = "BH")]
    Bahrain,
    #[serde(rename = "BD")]
    Bangladesh,
    #[serde(rename = "BB")]
    Barbados,
    #[serde(rename = "BY")]
    Belarus,
    #[serde(rename = "BE")]
    Belgium,
    #[serde(rename = "BZ")]
    Belize,
    #[serde(rename = "BJ")]
    Benin,
    #[serde(rename = "BM")]
    Bermuda,
    #[serde(rename = "BT")]
    Bhutan,
    #[serde(rename = "BO")]
    Bolivia,
    #[serde(rename = "BQ")]
    BonaireSintEustatiusSaba,
    #[serde(rename = "BA")]
    BosniaAndHerzegovina,
    #[serde(rename = "BW")]
    Botswana,
    #[serde(rename = "BV")]
    BouvetIsland,
    #[serde(rename = "BR")]
    Brazil,
    #[serde(rename = "IO")]
    BritishIndianOceanTerritory,
    #[serde(rename = "BN")]
    BruneiDarussalam,
    #[serde(rename = "BG")]
    Bulgaria,
    #[serde(rename = "BF")]
    BurkinaFaso,
    #[serde(rename = "BI")]
    Burundi,
    #[serde(rename = "KH")]
    Cambodia,
    #[serde(rename = "CM")]
    Cameroon,
    #[serde(rename = "CA")]
    Canada,
    #[serde(rename = "CV")]
    CapeVerde,
    #[serde(rename = "KY")]
    CaymanIslands,
    #[serde(rename = "CF")]
    CentralAfricanRepublic,
    #[serde(rename = "TD")]
    Chad,
    #[serde(rename = "CL")]
    Chile,
    #[serde(rename = "CN")]
    China,
    #[serde(rename = "CX")]
    ChristmasIsland,
    #[serde(rename = "CC")]
    CocosKeelingIslands,
    #[serde(rename = "CO")]
    Colombia,
    #[serde(rename = "KM")]
    Comoros,
    #[serde(rename = "CG")]
    Congo,
    #[serde(rename = "CD")]
    CongoDemocraticRepublic,
    #[serde(rename = "CK")]
    CookIslands,
    #[serde(rename = "CR")]
    CostaRica,
    #[serde(rename = "CI")]
    CoteDIvoire,
    #[serde(rename = "HR")]
    Croatia,
    #[serde(rename = "CU")]
    Cuba,
    #[serde(rename = "CW")]
    Curaçao,
    #[serde(rename = "CY")]
    Cyprus,
    #[serde(rename = "CZ")]
    CzechRepublic,
    #[serde(rename = "DK")]
    Denmark,
    #[serde(rename = "DJ")]
    Djibouti,
    #[serde(rename = "DM")]
    Dominica,
    #[serde(rename = "DO")]
    DominicanRepublic,
    #[serde(rename = "EC")]
    Ecuador,
    #[serde(rename = "EG")]
    Egypt,
    #[serde(rename = "SV")]
    ElSalvador,
    #[serde(rename = "GQ")]
    EquatorialGuinea,
    #[serde(rename = "ER")]
    Eritrea,
    #[serde(rename = "EE")]
    Estonia,
    #[serde(rename = "ET")]
    Ethiopia,
    #[serde(rename = "FK")]
    FalklandIslands,
    #[serde(rename = "FO")]
    FaroeIslands,
    #[serde(rename = "FJ")]
    Fiji,
    #[serde(rename = "FI")]
    Finland,
    #[serde(rename = "FR")]
    France,
    #[serde(rename = "GF")]
    FrenchGuiana,
    #[serde(rename = "PF")]
    FrenchPolynesia,
    #[serde(rename = "TF")]
    FrenchSouthernTerritories,
    #[serde(rename = "GA")]
    Gabon,
    #[serde(rename = "GM")]
    Gambia,
    #[serde(rename = "GE")]
    Georgia,
    #[serde(rename = "DE")]
    Germany,
    #[serde(rename = "GH")]
    Ghana,
    #[serde(rename = "GI")]
    Gibraltar,
    #[serde(rename = "GR")]
    Greece,
    #[serde(rename = "GL")]
    Greenland,
    #[serde(rename = "GD")]
    Grenada,
    #[serde(rename = "GP")]
    Guadeloupe,
    #[serde(rename = "GU")]
    Guam,
    #[serde(rename = "GT")]
    Guatemala,
    #[serde(rename = "GG")]
    Guernsey,
    #[serde(rename = "GN")]
    Guinea,
    #[serde(rename = "GW")]
    GuineaBissau,
    #[serde(rename = "GY")]
    Guyana,
    #[serde(rename = "HT")]
    Haiti,
    #[serde(rename = "HM")]
    HeardIslandMcdonaldIslands,
    #[serde(rename = "VA")]
    HolySeeVaticanCityState,
    #[serde(rename = "HN")]
    Honduras,
    #[serde(rename = "HK")]
    HongKong,
    #[serde(rename = "HU")]
    Hungary,
    #[serde(rename = "IS")]
    Iceland,
    #[serde(rename = "IN")]
    India,
    #[serde(rename = "ID")]
    Indonesia,
    #[serde(rename = "IR")]
    Iran,
    #[serde(rename = "IQ")]
    Iraq,
    #[serde(rename = "IE")]
    Ireland,
    #[serde(rename = "IM")]
    IsleOfMan,
    #[serde(rename = "IL")]
    Israel,
    #[serde(rename = "IT")]
    Italy,
    #[serde(rename = "JM")]
    Jamaica,
    #[serde(rename = "JP")]
    Japan,
    #[serde(rename = "JE")]
    Jersey,
    #[serde(rename = "JO")]
    Jordan,
    #[serde(rename = "KZ")]
    Kazakhstan,
    #[serde(rename = "KE")]
    Kenya,
    #[serde(rename = "KI")]
    Kiribati,
    #[serde(rename = "KR")]
    Korea,
    #[serde(rename = "KP")]
    KoreaDemocraticPeoplesRepublic,
    #[serde(rename = "KW")]
    Kuwait,
    #[serde(rename = "KG")]
    Kyrgyzstan,
    #[serde(rename = "LA")]
    LaoPeoplesDemocraticRepublic,
    #[serde(rename = "LV")]
    Latvia,
    #[serde(rename = "LB")]
    Lebanon,
    #[serde(rename = "LS")]
    Lesotho,
    #[serde(rename = "LR")]
    Liberia,
    #[serde(rename = "LY")]
    LibyanArabJamahiriya,
    #[serde(rename = "LI")]
    Liechtenstein,
    #[serde(rename = "LT")]
    Lithuania,
    #[serde(rename = "LU")]
    Luxembourg,
    #[serde(rename = "MO")]
    Macao,
    #[serde(rename = "MK")]
    Macedonia,
    #[serde(rename = "MG")]
    Madagascar,
    #[serde(rename = "MW")]
    Malawi,
    #[serde(rename = "MY")]
    Malaysia,
    #[serde(rename = "MV")]
    Maldives,
    #[serde(rename = "ML")]
    Mali,
    #[serde(rename = "MT")]
    Malta,
    #[serde(rename = "MH")]
    MarshallIslands,
    #[serde(rename = "MQ")]
    Martinique,
    #[serde(rename = "MR")]
    Mauritania,
    #[serde(rename = "MU")]
    Mauritius,
    #[serde(rename = "YT")]
    Mayotte,
    #[serde(rename = "MX")]
    Mexico,
    #[serde(rename = "FM")]
    Micronesia,
    #[serde(rename = "MD")]
    Moldova,
    #[serde(rename = "MC")]
    Monaco,
    #[serde(rename = "MN")]
    Mongolia,
    #[serde(rename = "ME")]
    Montenegro,
    #[serde(rename = "MS")]
    Montserrat,
    #[serde(rename = "MA")]
    Morocco,
    #[serde(rename = "MZ")]
    Mozambique,
    #[serde(rename = "MM")]
    Myanmar,
    #[serde(rename = "NA")]
    Namibia,
    #[serde(rename = "NR")]
    Nauru,
    #[serde(rename = "NP")]
    Nepal,
    #[serde(rename = "NL")]
    Netherlands,
    #[serde(rename = "NC")]
    NewCaledonia,
    #[serde(rename = "NZ")]
    NewZealand,
    #[serde(rename = "NI")]
    Nicaragua,
    #[serde(rename = "NE")]
    Niger,
    #[serde(rename = "NG")]
    Nigeria,
    #[serde(rename = "NU")]
    Niue,
    #[serde(rename = "NF")]
    NorfolkIsland,
    #[serde(rename = "MP")]
    NorthernMarianaIslands,
    #[serde(rename = "NO")]
    Norway,
    #[serde(rename = "OM")]
    Oman,
    #[serde(rename = "PK")]
    Pakistan,
    #[serde(rename = "PW")]
    Palau,
    #[serde(rename = "PS")]
    PalestinianTerritory,
    #[serde(rename = "PA")]
    Panama,
    #[serde(rename = "PG")]
    PapuaNewGuinea,
    #[serde(rename = "PY")]
    Paraguay,
    #[serde(rename = "PE")]
    Peru,
    #[serde(rename = "PH")]
    Philippines,
    #[serde(rename = "PN")]
    Pitcairn,
    #[serde(rename = "PL")]
    Poland,
    #[serde(rename = "PT")]
    Portugal,
    #[serde(rename = "PR")]
    PuertoRico,
    #[serde(rename = "QA")]
    Qatar,
    #[serde(rename = "RE")]
    Reunion,
    #[serde(rename = "RO")]
    Romania,
    #[serde(rename = "RU")]
    RussianFederation,
    #[serde(rename = "RW")]
    Rwanda,
    #[serde(rename = "BL")]
    SaintBarthelemy,
    #[serde(rename = "SH")]
    SaintHelena,
    #[serde(rename = "KN")]
    SaintKittsAndNevis,
    #[serde(rename = "LC")]
    SaintLucia,
    #[serde(rename = "MF")]
    SaintMartin,
    #[serde(rename = "PM")]
    SaintPierreAndMiquelon,
    #[serde(rename = "VC")]
    SaintVincentAndGrenadines,
    #[serde(rename = "WS")]
    Samoa,
    #[serde(rename = "SM")]
    SanMarino,
    #[serde(rename = "ST")]
    SaoTomeAndPrincipe,
    #[serde(rename = "SA")]
    SaudiArabia,
    #[serde(rename = "SN")]
    Senegal,
    #[serde(rename = "RS")]
    Serbia,
    #[serde(rename = "SC")]
    Seychelles,
    #[serde(rename = "SL")]
    SierraLeone,
    #[serde(rename = "SG")]
    Singapore,
    #[serde(rename = "SX")]
    SintMaarten,
    #[serde(rename = "SK")]
    Slovakia,
    #[serde(rename = "SI")]
    Slovenia,
    #[serde(rename = "SB")]
    SolomonIslands,
    #[serde(rename = "SO")]
    Somalia,
    #[serde(rename = "ZA")]
    SouthAfrica,
    #[serde(rename = "GS")]
    SouthGeorgiaAndSandwichIsl,
    #[serde(rename = "SS")]
    SouthSudan,
    #[serde(rename = "ES")]
    Spain,
    #[serde(rename = "LK")]
    SriLanka,
    #[serde(rename = "SD")]
    Sudan,
    #[serde(rename = "SR")]
    Suriname,
    #[serde(rename = "SJ")]
    SvalbardAndJanMayen,
    #[serde(rename = "SZ")]
    Swaziland,
    #[serde(rename = "SE")]
    Sweden,
    #[serde(rename = "CH")]
    Switzerland,
    #[serde(rename = "SY")]
    SyrianArabRepublic,
    #[serde(rename = "TW")]
    Taiwan,
    #[serde(rename = "TJ")]
    Tajikistan,
    #[serde(rename = "TZ")]
    Tanzania,
    #[serde(rename = "TH")]
    Thailand,
    #[serde(rename = "TL")]
    TimorLeste,
    #[serde(rename = "TG")]
    Togo,
    #[serde(rename = "TK")]
    Tokelau,
    #[serde(rename = "TO")]
    Tonga,
    #[serde(rename = "TT")]
    TrinidadAndTobago,
    #[serde(rename = "TN")]
    Tunisia,
    #[serde(rename = "TR")]
    Turkey,
    #[serde(rename = "TM")]
    Turkmenistan,
    #[serde(rename = "TC")]
    TurksAndCaicosIslands,
    #[serde(rename = "TV")]
    Tuvalu,
    #[serde(rename = "UG")]
    Uganda,
    #[serde(rename = "UA")]
    Ukraine,
    #[serde(rename = "AE")]
    UnitedArabEmirates,
    #[serde(rename = "GB")]
    UnitedKingdom,
    #[serde(rename = "US")]
    #[default]
    UnitedStates,
    #[serde(rename = "UM")]
    UnitedStatesOutlyingIslands,
    #[serde(rename = "UY")]
    Uruguay,
    #[serde(rename = "UZ")]
    Uzbekistan,
    #[serde(rename = "VU")]
    Vanuatu,
    #[serde(rename = "VE")]
    Venezuela,
    #[serde(rename = "VN")]
    Vietnam,
    #[serde(rename = "VG")]
    VirginIslandsBritish,
    #[serde(rename = "VI")]
    VirginIslandsUS,
    #[serde(rename = "WF")]
    WallisAndFutuna,
    #[serde(rename = "EH")]
    WesternSahara,
    #[serde(rename = "YE")]
    Yemen,
    #[serde(rename = "ZM")]
    Zambia,
    #[serde(rename = "ZW")]
    Zimbabwe,
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

        impl Default for $id_name {
            fn default() -> Self {
                Self::new(0)
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
