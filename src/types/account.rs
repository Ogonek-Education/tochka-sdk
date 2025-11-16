use crate::ExternalType;
use chrono::{DateTime, NaiveDate, Utc};
use codes_iso_4217::CurrencyCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[validate(length(min = 9))]
    pub customer_code: String,
    pub account_id: String,
    pub transit_account: Option<String>,
    pub status: AccountStatus,
    pub status_update_date_time: DateTime<Utc>,
    pub currency: CurrencyCode,
    pub account_type: ExternalType,
    pub account_sub_type: AccountSubType,
    pub registration_date: NaiveDate,
    pub account_details: Option<Vec<AccountDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountStatus {
    Enabled,
    Disabled,
    Deleted,
    ProForma,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountSubType {
    CreditCard,
    CurrentAccount,
    Loan,
    Mortgage,
    PrePaidCard,
    Savings,
    Special,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountDetail {
    #[validate(length(max = 40))]
    pub identification: String,
    pub name: String,
    pub scheme_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountIdentification {
    #[serde(rename = "RU.CBR.PAN")]
    RUCBRPAN,
    #[serde(rename = "RU.CBR.CellphoneNumber")]
    RUCBRCellphoneNumber,
    #[serde(rename = "RU.CBR.BBAN")]
    RUCBRBBAN,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashAccount {
    pub identification: Option<String>,
    pub scheme_name: AccountIdentification,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountPageData {
    pub account: Vec<Account>,
}
