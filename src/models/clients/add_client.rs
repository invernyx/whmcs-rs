use std::fmt::Write as _;

use base64::Engine;
use serde::{Deserialize, Serialize, Serializer};

use crate::models::{
    Country,
    clients::{ClientGroupId, ClientId, CustomField},
    system::CurrencyId,
    users::UserId,
};

#[allow(clippy::ref_option)]
fn serialize_whmcs_customfields<S>(
    value: &Option<Vec<CustomField>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        None => serializer.serialize_none(),
        Some(fields) => {
            let mut php = String::new();
            write!(&mut php, "a:{}:{{", fields.len()).expect("write to String");
            for f in fields {
                write!(&mut php, "i:{};", f.id).expect("write to String");
                write!(&mut php, "s:{}:\"", f.value.len()).expect("write to String");
                php.push_str(&f.value);
                php.push_str("\";");
            }
            php.push('}');
            serializer
                .serialize_str(&base64::engine::general_purpose::STANDARD.encode(php.as_bytes()))
        }
    }
}

#[derive(Debug, Serialize, Default)]
/// Parameters for adding a client on [`add_client`](crate::WhmcsClient::add_client).
#[serde(default)]
pub struct AddClientParams {
    /// The ID of the user that should own the client. Optional. When not provided, a new user will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_user_id: Option<UserId>,
    /// First name of the client to be created. Also used for the first name of the user when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified
    #[serde(rename = "firstname")]
    first_name: String,
    /// Last name of the client to be created. Also used for the last name of the user when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified.
    #[serde(rename = "lastname")]
    last_name: String,
    /// Company name of the client to be created.
    #[serde(rename = "companyname", skip_serializing_if = "Option::is_none")]
    company_name: Option<String>,
    /// Email address of the client to be created. Also used for the email of the user when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified.
    email: String,
    /// Address line 1 of the client to be created
    address1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Address line 2 of the client to be created
    address2: Option<String>,
    /// City of the client to be created
    city: String,
    /// State of the client to be created
    state: String,
    /// Postcode of the client to be created
    postcode: String,
    /// Country of the client to be created
    country: Country,
    /// Phone number of the client to be created
    #[serde(rename = "phonenumber")]
    phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client’s tax ID.
    tax_id: Option<String>,
    /// The password for the newly-created user account. Required when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified.
    #[serde(rename = "password2", skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    /// The ID of the security question in `tbladminsecurityquestions`. Required when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified.
    // TODO: Replace with security question ID
    #[serde(rename = "securityqid", skip_serializing_if = "Option::is_none")]
    security_question_id: Option<u32>,
    /// The security question answer for a newly-created user.
    #[serde(rename = "securityqans", skip_serializing_if = "Option::is_none")]
    security_question_answer: Option<String>,
    /// Currency ID from tblcurrencies.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    currency_id: Option<CurrencyId>,
    /// Client Group ID from tblclientgroups.
    #[serde(rename = "groupid", skip_serializing_if = "Option::is_none")]
    group_id: Option<ClientGroupId>,
    /// Custom fields and their values.
    #[serde(
        rename = "customfields",
        serialize_with = "serialize_whmcs_customfields",
        skip_serializing_if = "Option::is_none"
    )]
    custom_fields: Option<Vec<CustomField>>,
    /// Default language setting. Also used for the language of the user when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified. Provide the full name: ‘english’, ‘french’, etc….
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    /// The originating IP address for the request.
    #[serde(rename = "clientip", skip_serializing_if = "Option::is_none")]
    ip_address: Option<String>,
    /// Admin only notes
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
    /// Whether the client should opt-in to receiving marketing emails.
    #[serde(rename = "marketingoptin", skip_serializing_if = "Option::is_none")]
    marketing_opt_in: Option<bool>,
    /// Whether to send the client a welcome email. A true value will not send the email.
    #[serde(rename = "noemail", skip_serializing_if = "Option::is_none")]
    no_welcome_email: Option<bool>,
    /// Whether to enforce required fields. A true value will not enforce required fields. This does not apply to [`email`](`AddClientParams::email`) and [`password2`](`AddClientParams::password2`) when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified.
    #[serde(rename = "skipvalidation", skip_serializing_if = "Option::is_none")]
    skip_validation: Option<bool>,
    /// Credit card type. Provide full name: Visa, Mastercard, American Express, etc….
    #[serde(rename = "cardtype", skip_serializing_if = "Option::is_none")]
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    card_type: Option<String>,
    /// Credit card number.
    #[serde(rename = "cardnumber", skip_serializing_if = "Option::is_none")]
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    card_number: Option<String>,
    /// Credit card expiry date (format MMYY).
    #[serde(rename = "expdate", skip_serializing_if = "Option::is_none")]
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    card_expiry_date: Option<String>,
    /// Credit card start date (format MMYY).
    #[serde(rename = "startdate", skip_serializing_if = "Option::is_none")]
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    card_start_date: Option<String>,
    /// Credit card issue number (if applicable).
    #[serde(rename = "issuenumber", skip_serializing_if = "Option::is_none")]
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    card_issue_number: Option<String>,
    /// Credit card CVV number (will not be stored).
    #[serde(rename = "cvv", skip_serializing_if = "Option::is_none")]
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    card_cvv_number: Option<String>,
}

impl AddClientParams {
    /// The ID of the user that should own the client. Optional. When not provided, a new user will be created.
    #[must_use]
    pub fn owner_user_id(mut self, owner_user_id: impl Into<UserId>) -> Self {
        self.owner_user_id = Some(owner_user_id.into());
        self
    }

    /// First name of the client to be created. Also used for the first name of the user when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified
    #[must_use]
    pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = first_name.into();
        self
    }

    /// Last name of the client to be created. Also used for the last name of the user when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified.
    #[must_use]
    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = last_name.into();
        self
    }

    /// Company name of the client to be created.
    #[must_use]
    pub fn company_name(mut self, company_name: impl Into<String>) -> Self {
        self.company_name = Some(company_name.into());
        self
    }

    /// Email address of the client to be created. Also used for the email of the user when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified.
    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    /// Address line 1 of the client to be created
    #[must_use]
    pub fn address1(mut self, address1: impl Into<String>) -> Self {
        self.address1 = address1.into();
        self
    }

    /// Address line 2 of the client to be created
    #[must_use]
    pub fn address2(mut self, address2: impl Into<String>) -> Self {
        self.address2 = Some(address2.into());
        self
    }

    /// City of the client to be created
    #[must_use]
    pub fn city(mut self, city: impl Into<String>) -> Self {
        self.city = city.into();
        self
    }

    /// State of the client to be created
    #[must_use]
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = state.into();
        self
    }

    /// Postcode of the client to be created
    #[must_use]
    pub fn postcode(mut self, postcode: impl Into<String>) -> Self {
        self.postcode = postcode.into();
        self
    }

    /// Country of the client to be created
    #[must_use]
    pub fn country(mut self, country: impl Into<Country>) -> Self {
        self.country = country.into();
        self
    }

    /// Phone number of the client to be created
    #[must_use]
    pub fn phone_number(mut self, phone_number: impl Into<String>) -> Self {
        self.phone_number = phone_number.into();
        self
    }

    /// The client’s tax ID.
    #[must_use]
    pub fn tax_id(mut self, tax_id: impl Into<String>) -> Self {
        self.tax_id = Some(tax_id.into());
        self
    }

    /// The password for the newly-created user account. Required when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified.
    #[must_use]
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// The ID of the security question in `tbladminsecurityquestions`. Required when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified.
    #[must_use]
    pub fn security_question_id(mut self, security_question_id: impl Into<u32>) -> Self {
        self.security_question_id = Some(security_question_id.into());
        self
    }

    /// The security question answer for a newly-created user.
    #[must_use]
    pub fn security_question_answer(mut self, security_question_answer: impl Into<String>) -> Self {
        self.security_question_answer = Some(security_question_answer.into());
        self
    }

    /// Currency ID from tblcurrencies.
    #[must_use]
    pub fn currency_id(mut self, currency_id: impl Into<CurrencyId>) -> Self {
        self.currency_id = Some(currency_id.into());
        self
    }

    /// Client Group ID from tblclientgroups.
    #[must_use]
    pub fn group_id(mut self, group_id: impl Into<ClientGroupId>) -> Self {
        self.group_id = Some(group_id.into());
        self
    }

    /// Custom fields and their values.
    #[must_use]
    pub fn custom_fields(mut self, custom_fields: impl Into<Vec<CustomField>>) -> Self {
        self.custom_fields = Some(custom_fields.into());
        self
    }

    /// Default language setting. Also used for the language of the user when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified. Provide the full name: ‘english’, ‘french’, etc….
    #[must_use]
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// The originating IP address for the request.
    #[must_use]
    pub fn ip_address(mut self, ip_address: impl Into<String>) -> Self {
        self.ip_address = Some(ip_address.into());
        self
    }

    /// Admin only notes
    #[must_use]
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Whether the client should opt-in to receiving marketing emails.
    #[must_use]
    pub const fn marketing_opt_in(mut self, marketing_opt_in: bool) -> Self {
        self.marketing_opt_in = Some(marketing_opt_in);
        self
    }

    /// Whether to send the client a welcome email. A true value will not send the email.
    #[must_use]
    pub const fn no_welcome_email(mut self, no_welcome_email: bool) -> Self {
        self.no_welcome_email = Some(no_welcome_email);
        self
    }

    /// Whether to enforce required fields. A true value will not enforce required fields. This does not apply to [`email`](`AddClientParams::email`) and [`password2`](`AddClientParams::password2`) when [`owner_user_id`](`AddClientParams::owner_user_id`) is not specified.
    #[must_use]
    pub const fn skip_validation(mut self, skip_validation: bool) -> Self {
        self.skip_validation = Some(skip_validation);
        self
    }

    /// Credit card type. Provide full name: Visa, Mastercard, American Express, etc….
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    #[allow(deprecated)]
    #[must_use]
    pub fn card_type(mut self, card_type: impl Into<String>) -> Self {
        self.card_type = Some(card_type.into());
        self
    }

    /// Credit card number.
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    #[allow(deprecated)]
    #[must_use]
    pub fn card_number(mut self, card_number: impl Into<String>) -> Self {
        self.card_number = Some(card_number.into());
        self
    }

    /// Credit card expiry date (format MMYY).
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    #[allow(deprecated)]
    #[must_use]
    pub fn card_expiry_date(mut self, card_expiry_date: impl Into<String>) -> Self {
        self.card_expiry_date = Some(card_expiry_date.into());
        self
    }

    /// Credit card start date (format MMYY).
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    #[allow(deprecated)]
    #[must_use]
    pub fn card_start_date(mut self, card_start_date: impl Into<String>) -> Self {
        self.card_start_date = Some(card_start_date.into());
        self
    }

    /// Credit card issue number (if applicable).
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    #[allow(deprecated)]
    #[must_use]
    pub fn card_issue_number(mut self, card_issue_number: impl Into<String>) -> Self {
        self.card_issue_number = Some(card_issue_number.into());
        self
    }

    /// Credit card CVV number (will not be stored).
    #[deprecated(
        since = "0.1.2",
        note = "Deprecated in WHMCS 7.8.0, use AddPayMethod instead"
    )]
    #[allow(deprecated)]
    #[must_use]
    pub fn card_cvv_number(mut self, card_cvv_number: impl Into<String>) -> Self {
        self.card_cvv_number = Some(card_cvv_number.into());
        self
    }
}

#[derive(Debug, Deserialize)]
/// Response from [`add_client`](crate::WhmcsClient::add_client).
#[non_exhaustive]
pub struct AddClientResponse {
    /// The ID of the created client.
    #[serde(rename = "clientid")]
    pub client_id: ClientId,
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::models::clients::CustomField;

    use super::serialize_whmcs_customfields;

    fn encoded(custom_fields: Option<Vec<CustomField>>) -> String {
        #[derive(Serialize)]
        struct Row {
            #[serde(serialize_with = "serialize_whmcs_customfields")]
            custom_fields: Option<Vec<CustomField>>,
        }
        serde_json::to_value(Row { custom_fields })
            .unwrap()
            .get("custom_fields")
            .and_then(serde_json::Value::as_str)
            .unwrap()
            .to_owned()
    }

    #[test]
    fn custom_fields_base64_single_field_matches_php() {
        let fields = vec![CustomField {
            id: 183,
            value: "Google".into(),
        }];
        assert_eq!(
            encoded(Some(fields)),
            "YToxOntpOjE4MztzOjY6Ikdvb2dsZSI7fQ=="
        );
    }

    #[test]
    fn custom_fields_base64_two_fields_matches_php() {
        let fields = vec![
            CustomField {
                id: 183,
                value: "Google".into(),
            },
            CustomField {
                id: 184,
                value: "Other".into(),
            },
        ];
        assert_eq!(
            encoded(Some(fields)),
            "YToyOntpOjE4MztzOjY6Ikdvb2dsZSI7aToxODQ7czo1OiJPdGhlciI7fQ=="
        );
    }

    #[test]
    fn custom_fields_base64_empty_array_matches_php() {
        assert_eq!(encoded(Some(vec![])), "YTowOnt9");
    }

    #[test]
    fn custom_fields_embeds_quotes_and_backslashes_like_php() {
        let fields = vec![CustomField {
            id: 183,
            value: "a\"b\\c".into(),
        }];
        assert_eq!(encoded(Some(fields)), "YToxOntpOjE4MztzOjU6ImEiYlxjIjt9");
    }

    #[test]
    fn custom_fields_utf8_byte_length_matches_php() {
        let fields = vec![CustomField {
            id: 183,
            value: "café".into(),
        }];
        assert_eq!(encoded(Some(fields)), "YToxOntpOjE4MztzOjU6ImNhZsOpIjt9");
    }
}
