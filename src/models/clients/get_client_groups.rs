use serde::Deserialize;

use crate::models::{clients::ClientGroupId, deserialize_whmcs_bool, whmcs_nested_vec};

#[derive(Debug, Deserialize, Clone)]
/// Represents a client group in WHMCS. Returned by [`get_client_groups`](crate::WhmcsClient::get_client_groups).
pub struct ClientGroup {
    /// The ID number of the client group.
    pub id: ClientGroupId,
    /// The name of the client group.
    #[serde(rename = "groupname")]
    pub name: String,
    /// The color of the client group.
    #[serde(rename = "groupcolor")]
    pub color: u8,
    /// The discount percentage for the client group.
    #[serde(rename = "discountpercent")]
    pub discount_percentage: f32,
    /// Whether the client group is exempt from suspension term.
    #[serde(rename = "susptermexempt")]
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub suspension_term_exempt: bool,
    /// Whether the client group should have separate invoices.
    #[serde(rename = "separateinvoices")]
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub separate_invoices: bool,
}

whmcs_nested_vec!(deserialize_client_groups_nested, groups, ClientGroup);

#[derive(Debug, Deserialize)]
#[non_exhaustive]
/// Response from [`get_client_groups`](crate::WhmcsClient::get_client_groups).
pub struct GetClientGroupsResponse {
    /// The total number of results available
    #[serde(rename = "totalresults")]
    pub total_results: u32,
    /// The client group entries returned
    #[serde(default)]
    #[serde(
        rename = "groups",
        deserialize_with = "deserialize_client_groups_nested"
    )]
    pub groups: Vec<ClientGroup>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_client_groups_response_deserializes_nested_groups() {
        let j = r#"{
            "totalresults": 1,
            "groups": {
                "groups": [
                    {
                        "id": 0,
                        "groupname": "Default",
                        "groupcolor": 1,
                        "discountpercent": 0.0,
                        "susptermexempt": false,
                        "separateinvoices": false
                    }
                ]
            }
        }"#;
        let r: GetClientGroupsResponse = serde_json::from_str(j).unwrap();
        assert_eq!(r.total_results, 1);
        assert_eq!(r.groups.len(), 1);
        assert_eq!(r.groups[0].id, ClientGroupId::new(0));
        assert_eq!(r.groups[0].name, "Default");
        assert_eq!(r.groups[0].color, 1);
        assert!(!r.groups[0].suspension_term_exempt);
    }

    #[test]
    fn get_client_groups_response_empty_string_clients_field() {
        let j = r#"{"totalresults":0,"groups":""}"#;
        let r: GetClientGroupsResponse = serde_json::from_str(j).unwrap();
        assert_eq!(r.total_results, 0);
        assert!(r.groups.is_empty());
    }
}
