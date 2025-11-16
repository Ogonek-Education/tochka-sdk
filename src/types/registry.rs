use crate::{PaymentMode, PaymentStatus};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRegistryQuery {
    pub customer_code: String,
    pub merchant_id: String,
    pub payment_id: String,
    pub date: NaiveDate,
}

impl PaymentRegistryQuery {
    pub fn new(
        customer_code: impl Into<String>,
        merchant_id: impl Into<String>,
        payment_id: impl Into<String>,
        date: NaiveDate,
    ) -> Self {
        Self {
            customer_code: customer_code.into(),
            merchant_id: merchant_id.into(),
            payment_id: payment_id.into(),
            date,
        }
    }
}
#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registry {
    pub payment_type: PaymentMode,
    pub total_amount: f64,
    pub payment_id: Option<String>,
    pub payments: RegistryPayment,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryPayment {
    pub purpose: String,
    pub status: PaymentStatus,
    pub amount: f64,
    pub operation_id: Uuid,
    pub time: DateTime<Utc>,
    pub number: u32,
    pub commission: f64,
    pub enrollment_amount: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegistryPageData {
    pub registry: Vec<RegistryPayment>,
}
