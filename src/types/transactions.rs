use chrono::{DateTime, Utc};
use codes_iso_4217::CurrencyCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub account_id: String,
    pub pan: String,
    pub date_time: DateTime<Utc>,
    #[serde(rename = "Amount")]
    pub amount: Amount,
    #[serde(rename = "AccountAmount")]
    pub account_amount: AccountAmount,
    #[serde(rename = "TerminalData")]
    pub terminal_data: TerminalData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    pub amount: f64,
    pub currency: CurrencyCode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountAmount {
    pub amount: f64,
    pub currency: CurrencyCode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TerminalData {
    pub city: Option<String>,
    pub location: Option<String>,
    pub owner: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionPageData {
    pub transaction: Vec<Transaction>,
}
