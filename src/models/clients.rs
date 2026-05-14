use serde::{Deserialize, Serialize};

use crate::models::{
    WhmcsSorting, deserialize_whmcs_bool, u32_id, users::UserId, whmcs_nested_vec,
};

u32_id!(ClientGroupId);
u32_id!(ClientId);

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents the lifecycle status of a client.
pub enum ClientStatus {
    /// The client is active.
    Active,
    /// The client is inactive.
    Inactive,
    /// The client is closed.
    Closed,
}

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

#[derive(Debug, Serialize, Default)]
/// Parameters for obtaining the password for a client on [`get_client_password`](crate::WhmcsClient::get_client_password).
pub struct GetClientPasswordParams {
    /// The client ID to obtain the password for
    // WHMCS uses `userid` for the client ID, but we use `client_id` for consistency.
    #[serde(rename = "userid", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    /// The email address to obtain the password for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl GetClientPasswordParams {
    /// The userid to obtain the password for
    #[must_use]
    pub fn client_id(mut self, client_id: impl Into<ClientId>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// The email address to obtain the password for
    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
}

#[derive(Debug, Serialize, Default)]
/// Parameters for obtaining the details for a client on [`get_client_details`](crate::WhmcsClient::get_client_details).
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
    /// The client id to obtain the details for. `client_id` or `email` is required
    #[must_use]
    pub fn client_id(mut self, client_id: impl Into<ClientId>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// The email address to obtain the details for. `client_id` or `email` is required
    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Also return additional client statistics.
    #[must_use]
    pub const fn include_stats(mut self, include_stats: bool) -> Self {
        self.include_stats = Some(include_stats);
        self
    }
}

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

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
/// Represents a client's email preferences. Returned by [`get_client_details`](crate::WhmcsClient::get_client_details).
pub struct EmailPreference {
    /// Whether the client wants to receive general emails.
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub general: bool,
    /// Whether the client wants to receive invoice emails.
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub invoice: bool,
    /// Whether the client wants to receive support emails.
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub support: bool,
    /// Whether the client wants to receive product emails.
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub product: bool,
    /// Whether the client wants to receive domain emails.
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub domain: bool,
    /// Whether the client wants to receive affiliate emails.
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub affiliate: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
/// Represents custom fields on a client.
pub struct CustomField {
    /// The ID of the custom field.
    // TODO: Replace with custom field ID
    pub id: u32,
    /// The value of the custom field.
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
/// Represents a short data representation of a user belonging to a client.
/// Returned by [`get_client_details`](crate::WhmcsClient::get_client_details).
pub struct ClientUser {
    /// The ID of the user.
    pub id: UserId,
    /// The name of the user.
    pub name: String,
    /// The email address of the user.
    pub email: String,
    /// Whether the user is the owner of the client.
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub is_owner: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
/// Represents a client's statistics. Returned by [`get_client_details`](crate::WhmcsClient::get_client_details)
/// when `include_stats` is set to `true`.
pub struct ClientStats {
    /// The number of due invoices.
    #[serde(rename = "numdueinvoices")]
    pub due_invoices: u32,
    /// The balance of due invoices.
    #[serde(rename = "dueinvoicesbalance")]
    pub due_invoices_balance: String,
    /// Whether the client is in credit.
    #[serde(rename = "incredit", deserialize_with = "deserialize_whmcs_bool")]
    pub in_credit: bool,
    /// The credit balance of the client.
    #[serde(rename = "creditbalance")]
    pub credit_balance: String,
    /// The gross revenue of the client.
    #[serde(rename = "grossRevenue")]
    pub gross_revenue: String,
    /// The expenses of the client.
    pub expenses: String,
    /// The income of the client.
    pub income: String,
    /// The number of overdue invoices.
    #[serde(rename = "numoverdueinvoices")]
    pub overdue_invoices: u32,
    /// The balance of overdue invoices.
    #[serde(rename = "overdueinvoicesbalance")]
    pub overdue_invoices_balance: String,
    /// The number of draft invoices.
    #[serde(rename = "numDraftInvoices")]
    pub draft_invoices: u32,
    /// The balance of draft invoices.
    #[serde(rename = "draftInvoicesBalance")]
    pub draft_invoices_balance: String,
    /// The number of unpaid invoices.
    #[serde(rename = "numunpaidinvoices")]
    pub unpaid_invoices: u32,
    /// The balance of unpaid invoices.
    #[serde(rename = "unpaidinvoicesamount")]
    pub unpaid_invoices_balance: String,
    /// The number of paid invoices.
    #[serde(rename = "numpaidinvoices")]
    pub paid_invoices: u32,
    /// The balance of paid invoices.
    #[serde(rename = "paidinvoicesamount")]
    pub paid_invoices_balance: String,
    /// The number of cancelled invoices.
    #[serde(rename = "numcancelledinvoices")]
    pub cancelled_invoices: u32,
    /// The balance of cancelled invoices.
    #[serde(rename = "cancelledinvoicesamount")]
    pub cancelled_invoices_balance: String,
    /// The number of refunded invoices.
    #[serde(rename = "numrefundedinvoices")]
    pub refunded_invoices: u32,
    /// The balance of refunded invoices.
    #[serde(rename = "refundedinvoicesamount")]
    pub refunded_invoices_balance: String,
    /// The number of collections invoices.
    #[serde(rename = "numcollectionsinvoices")]
    pub collections_invoices: u32,
    /// The balance of collections invoices.
    #[serde(rename = "collectionsinvoicesamount")]
    pub collections_invoices_balance: String,
    /// The number of payment pending invoices.
    #[serde(rename = "numpaymentpendinginvoices")]
    pub payment_pending_invoices: u32,
    /// The balance of payment pending invoices.
    #[serde(rename = "paymentpendinginvoicesamount")]
    pub payment_pending_invoices_balance: String,
    /// The number of active hosting products.
    #[serde(rename = "productsnumactivehosting")]
    pub active_hosting_products: u32,
    /// The number of hosting products.
    #[serde(rename = "productsnumhosting")]
    pub hosting_products: u32,
    /// The number of active reseller products.
    #[serde(rename = "productsnumactivereseller")]
    pub active_reseller_products: u32,
    /// The number of reseller products.
    #[serde(rename = "productsnumreseller")]
    pub reseller_products: u32,
    /// The number of active servers.
    #[serde(rename = "productsnumactiveservers")]
    pub active_servers: u32,
    /// The number of servers.
    #[serde(rename = "productsnumservers")]
    pub servers: u32,
    /// The number of active other products.
    #[serde(rename = "productsnumactiveother")]
    pub active_other_products: u32,
    /// The number of other products.
    #[serde(rename = "productsnumother")]
    pub other_products: u32,
    /// The number of active products.
    #[serde(rename = "productsnumactive")]
    pub active_products: u32,
    /// The number of total products.
    #[serde(rename = "productsnumtotal")]
    pub total_products: u32,
    /// The number of active domains.
    #[serde(rename = "numactivedomains")]
    pub active_domains: u32,
    /// The number of domains.
    #[serde(rename = "numdomains")]
    pub domains: u32,
    /// The number of accepted quotes.
    #[serde(rename = "numacceptedquotes")]
    pub accepted_quotes: u32,
    /// The number of quotes.
    #[serde(rename = "numquotes")]
    pub quotes: u32,
    /// The number of tickets.
    #[serde(rename = "numtickets")]
    pub tickets: u32,
    /// The number of active tickets.
    #[serde(rename = "numactivetickets")]
    pub active_tickets: u32,
    /// The number of affiliate signups.
    #[serde(rename = "numaffiliatesignups")]
    pub affiliate_signups: u32,
    /// Whether the client is an affiliate.
    #[serde(rename = "isAffiliate", deserialize_with = "deserialize_whmcs_bool")]
    pub is_affiliate: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
/// Represents the details of a client. Returned by [`get_client_details`](crate::WhmcsClient::get_client_details).
pub struct ClientDetails {
    /// The ID of the client.
    pub client_id: ClientId,
    /// The ID of the owner user.
    pub owner_user_id: UserId,
    /// The ID of the user.
    #[serde(rename = "userid")]
    pub user_id: UserId,
    /// The UUID of the client.
    pub uuid: String,
    /// The first name of the client.
    #[serde(rename = "firstname")]
    pub first_name: String,
    /// The last name of the client.
    #[serde(rename = "lastname")]
    pub last_name: String,
    /// The full name of the client.
    #[serde(rename = "fullname")]
    pub full_name: String,
    /// The name of the company employing the client.
    #[serde(rename = "companyname")]
    pub company_name: String,
    /// The email address of the client.
    pub email: String,
    /// The first line of the client's address.
    pub address1: String,
    /// The second line of the client's address.
    pub address2: String,
    /// The city of the client's address.
    pub city: String,
    /// The full state of the client's address.
    #[serde(rename = "fullstate")]
    pub full_state: String,
    /// The state of the client's address.
    pub state: String,
    /// The postcode of the client's address.
    pub postcode: String,
    /// The country code of the client's address.
    #[serde(rename = "countrycode")]
    pub country_code: String,
    /// The country of the client's address.
    pub country: String,
    /// The phone number of the client.
    #[serde(rename = "phonenumber")]
    pub phone_number: String,
    /// The tax ID of the client.
    pub tax_id: String,
    /// The email preferences of the client.
    pub email_preferences: EmailPreference,
    /// The state code of the client's address.
    #[serde(rename = "statecode")]
    pub state_code: String,
    /// The country name of the client's address.
    #[serde(rename = "countryname")]
    pub country_name: String,
    /// The country code of the client's phone number.
    #[serde(rename = "phonecc")]
    pub phone_country_code: u32,
    /// The formatted phone number of the client.
    #[serde(rename = "phonenumberformatted")]
    pub phone_number_formatted: String,
    /// The telephone number of the client.
    #[serde(rename = "telephoneNumber")]
    pub telephone_number: String,
    /// The billing client ID of the client.
    #[serde(rename = "billingcid")]
    pub billing_cid: ClientId,
    /// The notes of the client.
    pub notes: String,
    /// The currency of the client.
    // TODO: Replace with currency ID
    pub currency: u8,
    /// The default gateway of the client.
    #[serde(rename = "defaultgateway")]
    pub default_gateway: String,
    /// The group ID of the client.
    #[serde(rename = "groupid")]
    pub group_id: ClientGroupId,
    /// The status of the client.
    pub status: ClientStatus,
    /// The credit of the client.
    pub credit: String,
    /// Whether the client is tax exempt.
    #[serde(rename = "taxexempt", deserialize_with = "deserialize_whmcs_bool")]
    pub tax_exempt: bool,
    /// Whether the client has an override for late fees.
    #[serde(rename = "latefeeoveride", deserialize_with = "deserialize_whmcs_bool")]
    pub late_fee_override: bool,
    /// Whether the client has an override for due notices.
    #[serde(
        rename = "overideduenotices",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub override_due_notices: bool,
    /// Whether the client has separate invoices.
    #[serde(
        rename = "separateinvoices",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub separate_invoices: bool,
    /// Whether the client has disabled auto CC.
    #[serde(rename = "disableautocc", deserialize_with = "deserialize_whmcs_bool")]
    pub disable_auto_cc: bool,
    /// Whether the client has opted out of email.
    #[serde(rename = "emailoptout", deserialize_with = "deserialize_whmcs_bool")]
    pub email_optout: bool,
    /// Whether the client has opted in to marketing emails.
    #[serde(
        rename = "marketing_emails_opt_in",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub marketing_emails_opt_in: bool,
    /// Whether the client has an override for auto close.
    #[serde(
        rename = "overrideautoclose",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub override_auto_close: bool,
    /// Whether the client allows single sign on.
    #[serde(
        rename = "allowSingleSignOn",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub allow_single_sign_on: bool,
    /// Whether the client's email address has been verified.
    #[serde(deserialize_with = "deserialize_whmcs_bool")]
    pub email_verified: bool,
    /// The language of the client.
    pub language: String,
    /// Whether the client is opted in to marketing emails.
    #[serde(
        rename = "isOptedInToMarketingEmails",
        deserialize_with = "deserialize_whmcs_bool"
    )]
    pub is_opted_in_to_marketing_emails: bool,
    /// The last login of the client.
    #[serde(rename = "lastlogin")]
    pub last_login: String,
    /// The currency code of the client.
    pub currency_code: String,
    /// The custom fields of the client.
    #[serde(rename = "customfields")]
    pub custom_fields: Option<Vec<CustomField>>,
    /// The users of the client.
    #[serde(deserialize_with = "deserialize_client_users_nested")]
    pub users: Vec<ClientUser>,
}

whmcs_nested_vec!(deserialize_client_groups_nested, groups, ClientGroup);
whmcs_nested_vec!(deserialize_clients_nested, client, Client);
whmcs_nested_vec!(deserialize_client_users_nested, user, ClientUser);

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

#[derive(Debug, Deserialize)]
#[non_exhaustive]
/// Response from [`get_client_password`](crate::WhmcsClient::get_client_password).
pub struct GetClientPasswordResponse {
    /// The encrypted password for the client
    pub password: String,
}

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

#[derive(Debug, Deserialize)]
#[non_exhaustive]
/// Response from [`get_client_details`](crate::WhmcsClient::get_client_details).
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
