use crate::{validate_phone, validate_tax_code};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Validate, Serialize, Debug)]
pub struct ReceiptClient {
    /// Для юрлица — название организации, для ИП и физического лица — ФИО
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(email)]
    /// Email покупателя, на который будет отправлен чек
    pub email: String,
    /// Телефон пользователя для отправки чека.
    #[validate(custom(function = "validate_phone"))]
    pub phone: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TaxSystemCode {
    Osn,
    UsnIncome,
    UsnIncomeOutcome,
    Esn,
    Patent,
    Envd,
}
