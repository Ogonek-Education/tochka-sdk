use crate::TransactionStatement;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statement {
    #[validate(length(max = 40))]
    pub account_id: String,
    #[validate(length(max = 40))]
    pub statement_id: Option<String>,
    pub status: StatementStatus,
    pub start_date_time: NaiveDate,
    pub end_date_time: NaiveDate,
    pub creation_date_time: DateTime<Utc>,
    pub start_date_balance: Option<f64>,
    pub end_date_balance: Option<f64>,
    #[serde(rename = "Transaction")]
    pub transaction: Option<Vec<TransactionStatement>>,
}

#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatementPayload {
    #[validate(length(max = 40))]
    pub account_id: String,
    pub start_date_time: NaiveDate,
    pub end_date_time: NaiveDate,
}

#[derive(Serialize, Debug, Deserialize, Default)]
pub enum StatementStatus {
    #[default]
    Created,
    Processing,
    Error,
    Ready,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatementPageData {
    pub statement: Vec<Statement>,
}
