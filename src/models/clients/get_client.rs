use serde::{Deserialize, Serialize};

use crate::{
    WhmcsSorting,
    models::{
        clients::{ClientGroupId, ClientId, ClientStatus},
        whmcs_nested_vec,
    },
};

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
/// Represents the column to order by when retrieving clients.
pub enum ClientOrderBy {
    /// Ordering by client's unique ID number.
    Id,
    /// Ordering by client first name.
    FirstName,
    /// Ordering by client last name.
    LastName,
    /// Ordering by client company name.
    CompanyName,
    /// Ordering by client email address.
    Email,
    /// Ordering by client group ID.
    GroupId,
    /// Ordering by client creation date.
    DateCreated,
    /// Ordering by client status.
    Status,
}

#[derive(Debug, Serialize, Default)]
/// Parameters for filtering and sorting the clients on [`get_clients`](crate::WhmcsClient::get_clients).
pub struct GetClientParams {
    /// The offset for the returned log data (default: 0)
    #[serde(rename = "limitstart", skip_serializing_if = "Option::is_none")]
    pub limit_start: Option<u32>,
    /// The number of records to return (default: 25)
    #[serde(rename = "limitnum", skip_serializing_if = "Option::is_none")]
    pub limit_num: Option<u32>,
    /// The direction to sort the results. ASC or DESC. Default: ASC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorting: Option<WhmcsSorting>,
    /// Optional desired Client Status. ‘Active’, ‘Inactive’, or ‘Closed’.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClientStatus>,
    /// The search term to look for at the start of email, firstname, lastname, fullname or companyname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// The column to order by. id, firstname, lastname, companyname, email, groupid, datecreated, status
    #[serde(rename = "orderby", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<ClientOrderBy>,
}

impl GetClientParams {
    /// The offset for the returned log data (default: 0)
    #[must_use]
    pub const fn limit_start(mut self, limit_start: u32) -> Self {
        self.limit_start = Some(limit_start);
        self
    }

    /// The number of records to return (default: 25)
    #[must_use]
    pub const fn limit_num(mut self, limit_num: u32) -> Self {
        self.limit_num = Some(limit_num);
        self
    }

    /// The direction to sort the results. ASC or DESC. Default: ASC
    #[must_use]
    pub const fn sorting(mut self, sorting: WhmcsSorting) -> Self {
        self.sorting = Some(sorting);
        self
    }

    /// Optional desired Client Status. ‘Active’, ‘Inactive’, or ‘Closed’.
    #[must_use]
    pub const fn status(mut self, status: ClientStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// The search term to look for at the start of email, firstname, lastname, fullname or companyname
    #[must_use]
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }

    /// The column to order by. id, firstname, lastname, companyname, email, groupid, datecreated, status
    #[must_use]
    pub const fn order_by(mut self, order_by: ClientOrderBy) -> Self {
        self.order_by = Some(order_by);
        self
    }
}

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
/// Represents a client in WHMCS. Returned by [`get_clients`](crate::WhmcsClient::get_clients).
pub struct Client {
    /// A client's unique ID number.
    pub id: ClientId,
    /// A client's first name.
    #[serde(rename = "firstname")]
    pub first_name: String,
    /// A client's last name.
    #[serde(rename = "lastname")]
    pub last_name: String,
    /// A client's email address.
    pub email: String,
    /// The name of the company employing a client.
    #[serde(rename = "companyname")]
    pub company_name: String,
    /// Initial creation of the client data.
    ///
    /// Note: If the user was created before WHMCS 6.0.0, this will be set to 0000-00-00 00:00:00
    #[serde(rename = "datecreated")]
    pub date_created: String,
    /// The ID number of the group that a client belongs to.
    #[serde(rename = "groupid")]
    pub group_id: ClientGroupId,
    /// A client's status, either [`ClientStatus::Active`], [`ClientStatus::Inactive`], or [`ClientStatus::Closed`].
    #[serde(rename = "status")]
    pub status: ClientStatus,
}

whmcs_nested_vec!(deserialize_clients_nested, client, Client);

#[derive(Debug, Deserialize)]
#[non_exhaustive]
/// Response from [`get_clients`](crate::WhmcsClient::get_clients).
pub struct GetClientsResponse {
    /// The total number of results available
    #[serde(rename = "totalresults")]
    pub total_results: u32,
    /// The starting number for the returned results
    #[serde(rename = "startnumber")]
    pub start_number: u32,
    /// The number of results returned
    #[serde(rename = "numreturned")]
    pub number_returned: u32,
    /// The client entries returned
    #[serde(default)]
    #[serde(rename = "clients", deserialize_with = "deserialize_clients_nested")]
    pub clients: Vec<Client>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WhmcsSorting;

    #[test]
    fn get_client_params_serializes_expected_keys() {
        let p = GetClientParams::default()
            .limit_start(10)
            .limit_num(25)
            .sorting(WhmcsSorting::Descending)
            .status(ClientStatus::Active)
            .search("admin@")
            .order_by(ClientOrderBy::Email);
        let v = serde_json::to_value(&p).unwrap();
        assert_eq!(v["limitstart"], 10);
        assert_eq!(v["limitnum"], 25);
        assert_eq!(v["sorting"], "DESC");
        assert_eq!(v["status"], "Active");
        assert_eq!(v["search"], "admin@");
        assert_eq!(v["orderby"], "email");
    }

    #[test]
    fn get_clients_response_deserializes_nested_clients() {
        let j = r#"{
            "totalresults": 1,
            "startnumber": 0,
            "numreturned": 1,
            "clients": {
                "client": [
                    {
                        "id": 42,
                        "firstname": "A",
                        "lastname": "B",
                        "email": "a@b.c",
                        "companyname": "Co",
                        "datecreated": "2020-01-01",
                        "groupid": 0,
                        "status": "Inactive"
                    }
                ]
            }
        }"#;
        let r: GetClientsResponse = serde_json::from_str(j).unwrap();
        assert_eq!(r.total_results, 1);
        assert_eq!(r.start_number, 0);
        assert_eq!(r.number_returned, 1);
        assert_eq!(r.clients.len(), 1);
        assert_eq!(r.clients[0].id, ClientId::new(42));
        assert_eq!(r.clients[0].first_name, "A");
        assert_eq!(r.clients[0].status, ClientStatus::Inactive);
        assert_eq!(r.clients[0].group_id, ClientGroupId::new(0));
    }

    #[test]
    fn get_clients_response_empty_clients_string() {
        let j = r#"{"totalresults":0,"startnumber":0,"numreturned":0,"clients":""}"#;
        let r: GetClientsResponse = serde_json::from_str(j).unwrap();
        assert!(r.clients.is_empty());
    }
}
