use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Amount;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub account_id: String,
    pub credit_debit_indicator: CreditDebitIndicator,
    #[serde(rename = "type")]
    pub balance_type: BalanceType,
    pub date_time: DateTime<Utc>,
    #[serde(rename = "Amount")]
    pub amount: Amount,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CreditDebitIndicator {
    Credit,
    Debit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BalanceType {
    OpeningAvailable,
    ClosingAvailable,
    Expected,
    OverdraftAvailable,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BalancePageData {
    pub balance: Vec<Balance>,
}
