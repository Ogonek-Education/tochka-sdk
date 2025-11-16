use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: String,
    pub created_at: String,
    pub legal_id: String,
    pub status: AccountStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDetail {
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
