use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Amount;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum CreditDebitIndicator {
    Credit,
    Debit,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BalanceType {
    OpeningAvailable,
    ClosingAvailable,
    Expected,
    OverdraftAvailable,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BalancePageData {
    pub balance: Vec<Balance>,
}
