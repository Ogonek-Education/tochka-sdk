use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contractor {
    pub inn: Option<String>,
    pub kpp: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractorBank {
    pub account_identification: Option<String>,
    pub identification: Option<String>,
    pub name: Option<String>,
    pub scheme_name: FinancialInstitutionIdentification,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FinancialInstitutionIdentification {
    #[serde(rename = "RU.CBR.BICFI")]
    RuCbrBicfi,

    #[serde(rename = "RU.CBR.BIK")]
    RuCbrBik,
}
