use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Refund {
    pub is_refund: bool,
    pub operation_id: uuid::Uuid,
    pub amount: f64,
    pub date: NaiveDate,
    pub order_id: String,
}

#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundPayload {
    #[validate(range(min = 0.0))]
    pub amount: f64,
}
