use serde::{Deserialize, Serialize};

use crate::models::{
    WhmcsSorting, deserialize_whmcs_bool, u32_id, users::UserId, whmcs_nested_vec,
};

u32_id!(ClientGroupId);
u32_id!(ClientId);

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientStatus {
    Active,
    Inactive,
    Closed,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ClientOrderBy {
    Id,
    FirstName,
    LastName,
    CompanyName,
    Email,
    GroupId,
    DateCreated,
    Status,
}

#[derive(Debug, Serialize, Default)]
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
    pub fn limit_start(mut self, limit_start: u32) -> Self {
        self.limit_start = Some(limit_start);
        self
    }

    pub fn limit_num(mut self, limit_num: u32) -> Self {
        self.limit_num = Some(limit_num);
        self
    }

    pub fn sorting(mut self, sorting: WhmcsSorting) -> Self {
        self.sorting = Some(sorting);
        self
    }

    pub fn status(mut self, status: ClientStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }

    pub fn order_by(mut self, order_by: ClientOrderBy) -> Self {
        self.order_by = Some(order_by);
        self
    }
}

#[derive(Debug, Serialize, Default)]
pub struct GetClientPasswordParams {
    /// The userid to obtain the password for
    #[serde(rename = "userid", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    /// The email address to obtain the password for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl GetClientPasswordParams {
    pub fn client_id(mut self, client_id: impl Into<ClientId>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
}

#[derive(Debug, Serialize, Default)]
pub struct GetClientDetailsParams {
    /// The client id to obtain the details for. `client_id` or `email` is required
    #[serde(rename = "clientid", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    /// The email address to obtain the details for. `client_id` or `email` is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Also return additional client statistics.
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub include_stats: Option<bool>,
}

impl GetClientDetailsParams {
    pub fn client_id(mut self, client_id: impl Into<ClientId>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn include_stats(mut self, include_stats: bool) -> Self {
        self.include_stats = Some(include_stats);
        self
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClientGroup {
    pub id: ClientGroupId,
    #[serde(rename = "groupname")]
    pub name: String,
    #[serde(rename = "groupcolor")]
    pub color: u8,
    #[serde(rename = "discountpercent")]
    pub discount_percentage: f32,
    #[serde(rename = "susptermexempt")]
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub suspension_term_exempt: bool,
    #[serde(rename = "separateinvoices")]
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub separate_invoices: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
pub struct Client {
    pub id: ClientId,
    #[serde(rename = "firstname")]
    pub first_name: String,
    #[serde(rename = "lastname")]
    pub last_name: String,
    pub email: String,
    #[serde(rename = "companyname")]
    pub company_name: String,
    #[serde(rename = "datecreated")]
    pub date_created: String,
    #[serde(rename = "groupid")]
    pub group_id: ClientGroupId,
    #[serde(rename = "status")]
    pub status: ClientStatus,
}

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
pub struct EmailPreference {
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub general: bool,
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub invoice: bool,
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub support: bool,
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub product: bool,
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub domain: bool,
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub affiliate: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
pub struct CustomField {
    // TODO: Replace with custom field ID
    pub id: u32,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
pub struct ClientUser {
    pub id: UserId,
    pub name: String,
    pub email: String,
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub is_owner: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
pub struct ClientStats {
    #[serde(rename = "numdueinvoices")]
    pub due_invoices: u32,
    #[serde(rename = "dueinvoicesbalance")]
    pub due_invoices_balance: String,
    #[serde(rename = "incredit", deserialize_with = "deserialize_whmcs_bool")]
    pub in_credit: bool,
    #[serde(rename = "creditbalance")]
    pub credit_balance: String,
    #[serde(rename = "grossRevenue")]
    pub gross_revenue: String,
    pub expenses: String,
    pub income: String,
    #[serde(rename = "numoverdueinvoices")]
    pub overdue_invoices: u32,
    #[serde(rename = "overdueinvoicesbalance")]
    pub overdue_invoices_balance: String,
    #[serde(rename = "numDraftInvoices")]
    pub draft_invoices: u32,
    #[serde(rename = "draftInvoicesBalance")]
    pub draft_invoices_balance: String,
    #[serde(rename = "numunpaidinvoices")]
    pub unpaid_invoices: u32,
    #[serde(rename = "unpaidinvoicesamount")]
    pub unpaid_invoices_balance: String,
    #[serde(rename = "numpaidinvoices")]
    pub paid_invoices: u32,
    #[serde(rename = "paidinvoicesamount")]
    pub paid_invoices_balance: String,
    #[serde(rename = "numcancelledinvoices")]
    pub cancelled_invoices: u32,
    #[serde(rename = "cancelledinvoicesamount")]
    pub cancelled_invoices_balance: String,
    #[serde(rename = "numrefundedinvoices")]
    pub refunded_invoices: u32,
    #[serde(rename = "refundedinvoicesamount")]
    pub refunded_invoices_balance: String,
    #[serde(rename = "numcollectionsinvoices")]
    pub collections_invoices: u32,
    #[serde(rename = "collectionsinvoicesamount")]
    pub collections_invoices_balance: String,
    #[serde(rename = "numpaymentpendinginvoices")]
    pub payment_pending_invoices: u32,
    #[serde(rename = "paymentpendinginvoicesamount")]
    pub payment_pending_invoices_balance: String,
    #[serde(rename = "productsnumactivehosting")]
    pub active_hosting_products: u32,
    #[serde(rename = "productsnumhosting")]
    pub hosting_products: u32,
    #[serde(rename = "productsnumactivereseller")]
    pub active_reseller_products: u32,
    #[serde(rename = "productsnumreseller")]
    pub reseller_products: u32,
    #[serde(rename = "productsnumactiveservers")]
    pub active_servers: u32,
    #[serde(rename = "productsnumservers")]
    pub servers: u32,
    #[serde(rename = "productsnumactiveother")]
    pub active_other_products: u32,
    #[serde(rename = "productsnumother")]
    pub other_products: u32,
    #[serde(rename = "productsnumactive")]
    pub active_products: u32,
    #[serde(rename = "productsnumtotal")]
    pub total_products: u32,
    #[serde(rename = "numactivedomains")]
    pub active_domains: u32,
    #[serde(rename = "numdomains")]
    pub domains: u32,
    #[serde(rename = "numacceptedquotes")]
    pub accepted_quotes: u32,
    #[serde(rename = "numquotes")]
    pub quotes: u32,
    #[serde(rename = "numtickets")]
    pub tickets: u32,
    #[serde(rename = "numactivetickets")]
    pub active_tickets: u32,
    #[serde(rename = "numaffiliatesignups")]
    pub affiliate_signups: u32,
    #[serde(rename = "isAffiliate", deserialize_with = "deserialize_whmcs_bool")]
    pub is_affiliate: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
pub struct ClientDetails {
    pub client_id: ClientId,
    pub owner_user_id: UserId,
    #[serde(rename = "userid")]
    pub user_id: UserId,
    pub id: ClientId,
    pub uuid: String,
    #[serde(rename = "firstname")]
    pub first_name: String,
    #[serde(rename = "lastname")]
    pub last_name: String,
    #[serde(rename = "fullname")]
    pub full_name: String,
    #[serde(rename = "companyname")]
    pub company_name: String,
    pub email: String,
    pub address1: String,
    pub address2: String,
    pub city: String,
    #[serde(rename = "fullstate")]
    pub full_state: String,
    pub state: String,
    pub postcode: String,
    #[serde(rename = "countrycode")]
    pub country_code: String,
    pub country: String,
    #[serde(rename = "phonenumber")]
    pub phone_number: String,
    pub tax_id: String,
    pub email_preferences: EmailPreference,
    #[serde(rename = "statecode")]
    pub state_code: String,
    #[serde(rename = "countryname")]
    pub country_name: String,
    #[serde(rename = "phonecc")]
    pub phone_country_code: u32,
    #[serde(rename = "phonenumberformatted")]
    pub phone_number_formatted: String,
    #[serde(rename = "telephoneNumber")]
    pub telephone_number: String,
    #[serde(rename = "billingcid")]
    pub billing_cid: ClientId,
    pub notes: String,
    // TODO: Replace with currency ID
    pub currency: u8,
    #[serde(rename = "defaultgateway")]
    pub default_gateway: String,
    #[serde(rename = "groupid")]
    pub group_id: ClientGroupId,
    pub status: ClientStatus,
    pub credit: String,
    #[serde(rename = "taxexempt", deserialize_with = "deserialize_whmcs_bool")]
    pub tax_exempt: bool,
    #[serde(rename = "latefeeoveride", deserialize_with = "deserialize_whmcs_bool")]
    pub late_fee_override: bool,
    #[serde(
        rename = "overideduenotices",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub override_due_notices: bool,
    #[serde(
        rename = "separateinvoices",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub separate_invoices: bool,
    #[serde(rename = "disableautocc", deserialize_with = "deserialize_whmcs_bool")]
    pub disable_auto_cc: bool,
    #[serde(rename = "emailoptout", deserialize_with = "deserialize_whmcs_bool")]
    pub email_optout: bool,
    #[serde(
        rename = "marketing_emails_opt_in",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub marketing_emails_opt_in: bool,
    #[serde(
        rename = "overrideautoclose",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub override_auto_close: bool,
    #[serde(
        rename = "allowSingleSignOn",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub allow_single_sign_on: bool,
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub email_verified: bool,
    pub language: String,
    #[serde(
        rename = "isOptedInToMarketingEmails",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub is_opted_in_to_marketing_emails: bool,
    #[serde(rename = "lastlogin")]
    pub last_login: String,
    pub currency_code: String,
    #[serde(rename = "customfields")]
    pub custom_fields: Option<Vec<CustomField>>,
    #[serde(deserialize_with = "deserialize_client_users_nested")]
    pub users: Vec<ClientUser>,
}

whmcs_nested_vec!(deserialize_client_groups_nested, groups, ClientGroup);
whmcs_nested_vec!(deserialize_clients_nested, client, Client);
whmcs_nested_vec!(deserialize_client_users_nested, user, ClientUser);

#[derive(Debug, Deserialize)]
#[non_exhaustive]
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

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct GetClientPasswordResponse {
    /// The encrypted password for the client
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
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

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct GetClientDetailsResponse {
    /// The information on the client
    pub client: ClientDetails,
    /// Additional statistics for the client returned
    pub stats: Option<ClientStats>,
}

#[cfg(test)]
mod clients_model_tests {
    use super::*;

    #[test]
    fn get_clients_response_deserializes_nested_clients() {
        let json = r#"{
            "totalresults": 2,
            "startnumber": 0,
            "numreturned": 2,
            "clients": {
                "client": [
                    {
                        "id": 1,
                        "firstname": "A",
                        "lastname": "B",
                        "email": "a@b.c",
                        "companyname": "",
                        "datecreated": "2020-01-01",
                        "groupid": 0,
                        "status": "Active"
                    },
                    {
                        "id": 2,
                        "firstname": "C",
                        "lastname": "D",
                        "email": "c@d.e",
                        "companyname": "Co",
                        "datecreated": "2020-01-02",
                        "groupid": 1,
                        "status": "Inactive"
                    }
                ]
            }
        }"#;
        let r: GetClientsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.total_results, 2);
        assert_eq!(r.number_returned, 2);
        assert_eq!(r.clients.len(), 2);
        assert_eq!(r.clients[0].email, "a@b.c");
        assert_eq!(r.clients[1].status, ClientStatus::Inactive);
    }

    #[test]
    fn get_client_params_serializes_expected_keys() {
        let p = GetClientParams::default()
            .search("needle")
            .limit_start(10)
            .limit_num(5)
            .sorting(crate::models::WhmcsSorting::Descending)
            .status(ClientStatus::Active)
            .order_by(ClientOrderBy::Email);
        let v = serde_json::to_value(&p).unwrap();
        assert_eq!(v["search"], "needle");
        assert_eq!(v["limitstart"], 10);
        assert_eq!(v["limitnum"], 5);
        assert_eq!(v["sorting"], "DESC");
        assert_eq!(v["status"], "Active");
        assert_eq!(v["orderby"], "email");
    }

    #[test]
    fn get_client_details_params_serializes_clientid_and_stats() {
        let p = GetClientDetailsParams::default()
            .client_id(ClientId::new(42))
            .include_stats(true);
        let v = serde_json::to_value(&p).unwrap();
        assert_eq!(v["clientid"], 42);
        assert_eq!(v["stats"], true);
        assert!(v.get("email").is_none());
    }

    #[test]
    fn client_id_new_and_from_u32() {
        assert_eq!(ClientId::from(7_u32), ClientId::new(7));
    }
}
