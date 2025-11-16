use crate::PaymentMode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Retailer {
    pub status: RetailerStatus,
    pub is_active: bool,
    pub mcc: String,
    pub rate: f64,
    pub name: String,
    pub url: String,
    pub merchant_id: String,
    pub terminal_id: String,
    pub payment_modes: Vec<PaymentMode>,
    pub cashbox: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RetailerStatus {
    #[serde(rename = "NEW")]
    NEW,
    #[serde(rename = "ADDRESS_DADATA")]
    ADDRESSDADATA,
    #[serde(rename = "OPEN_ACCOUNT")]
    OPENACCOUNT,
    #[serde(rename = "TWPG_SENDED")]
    TWPGSENDED,
    #[serde(rename = "RETAILER_CREATED")]
    RETAILERCREATED,
    #[serde(rename = "TERMINAL_CREATED")]
    TERMINALCREATED,
    #[serde(rename = "FILE_SENT")]
    FILESENT,
    #[serde(rename = "REG")]
    REG,
    #[serde(rename = "CLOSE")]
    CLOSE,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RetailerPageData {
    pub retailer: Vec<Retailer>,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RetailerQuery {
    pub customer_code: String,
}

impl RetailerQuery {
    pub fn new(customer_code: impl Into<String>) -> Self {
        Self {
            customer_code: customer_code.into(),
        }
    }
}
