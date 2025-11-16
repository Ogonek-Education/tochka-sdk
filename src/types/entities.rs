use crate::{validate_phone, validate_tax_code};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

#[derive(Deserialize, Validate, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Supplier {
    #[validate(custom(function = "validate_phone"))]
    pub phone: String,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(custom(function = "validate_tax_code"))]
    pub tax_code: String,
}

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

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    /// Уникальный код клиента
    pub customer_code: String,
    /// Тип клиент (физическое или юридическое лицо)
    pub customer_type: ExternalType,
    /// Признак резидента
    pub is_resident: bool,
    /// ИНН
    #[validate(custom(function = validate_tax_code))]
    pub tax_code: Option<String>,
    /// Индивидуальный Предприниматель Тест
    pub full_name: String,
    /// ИП Тест
    pub short_name: Option<String>,
    /// КПП
    pub kpp: Option<String>,
    /// ОГРН или ОГРНИМ
    pub customer_ogrn: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, EnumString, Display, PartialEq)]
pub enum ExternalType {
    Business,
    Personal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomerPageData {
    pub customer: Vec<Customer>,
}
