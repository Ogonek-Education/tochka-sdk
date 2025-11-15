use crate::validate_tax_code;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

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

#[derive(Debug, Deserialize, Serialize, EnumString, Display, PartialEq)]
pub enum ExternalType {
    Business,
    Personal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinancialInstitutionIdentificationEnum {
    #[serde(rename = "RU.CBR.BICFI")]
    RuCbrBicfi,

    #[serde(rename = "RU.CBR.BIK")]
    RuCbrBik,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomerPageData {
    pub customer: Vec<Customer>,
}
