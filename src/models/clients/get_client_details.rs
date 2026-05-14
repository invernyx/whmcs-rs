use serde::{Deserialize, Serialize};

use crate::models::{
    clients::{ClientGroupId, ClientId, ClientStatus},
    deserialize_whmcs_bool,
    users::UserId,
    whmcs_nested_vec,
};

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

whmcs_nested_vec!(deserialize_client_users_nested, user, ClientUser);

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
mod tests {
    use super::*;

    #[test]
    fn get_client_details_params_serializes_clientid_and_stats() {
        let p = GetClientDetailsParams::default()
            .client_id(ClientId::new(9))
            .email("c@d.e")
            .include_stats(true);
        let v = serde_json::to_value(&p).unwrap();
        assert_eq!(v["clientid"], 9);
        assert_eq!(v["email"], "c@d.e");
        assert_eq!(v["stats"], true);
    }

    #[test]
    fn email_preference_deserializes_string_bools() {
        let j = r#"{"general":"1","invoice":"0","support":"true","product":"0","domain":"0","affiliate":"no"}"#;
        let e: EmailPreference = serde_json::from_str(j).unwrap();
        assert!(e.general);
        assert!(!e.invoice);
        assert!(e.support);
        assert!(!e.affiliate);
    }

    #[test]
    fn get_client_details_response_deserializes_from_fixture() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/get_client_details_minimal.json"
        ));
        let r: GetClientDetailsResponse = serde_json::from_str(raw).unwrap();
        assert_eq!(r.client.client_id, ClientId::new(1));
        assert_eq!(r.client.email, "jane@example.com");
        assert_eq!(r.client.users.len(), 1);
        assert_eq!(r.client.users[0].id.as_u32(), 1);
        assert!(r.client.users[0].is_owner);
        assert!(r.stats.is_none());
    }
}
