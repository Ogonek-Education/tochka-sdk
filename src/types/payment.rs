use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Полностью ли оплачен
#[derive(Serialize, Debug, Deserialize, EnumString, Display)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethod {
    FullPayment,
    FullPrepayment,
}

/// Статус платежа
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    Created,
    Approved,
    #[serde(rename = "ON-REFUND")]
    OnRefund,
    Refunded,
    Expired,
    RefundedPartially,
    Authorized,
    WaitFullPayment,
}

/// Как клиент платит – способ оплаты
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMode {
    Sbp,
    Card,
    Tinkoff,
    Dolyame,
}

/// Признак предмета платежа
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PaymentObject {
    Goods,
    Service,
    Work,
}
