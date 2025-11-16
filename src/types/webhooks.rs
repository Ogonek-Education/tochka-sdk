use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

/// RU: Настройка вебхуков приложения. EN: Application webhook configuration.
#[derive(Serialize, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    /// RU: Список подписок. EN: Subscribed webhook types.
    pub webhooks_list: Vec<WebhookType>,
    #[validate(length(max = 2083))]
    /// RU: URL для отправки вебхуков. EN: Callback URL.
    pub url: String,
}

/// RU: Типы вебхуков. EN: Webhook event types.
#[derive(Serialize, Debug, Deserialize, EnumString, Display)]
#[serde(rename_all = "camelCase")]
pub enum WebhookType {
    IncomingPayment,
    OutgoingPayment,
    IncomingSbpPayment,
    AcquiringInternetPayment,
    IncomingSbpB2BPayment,
}
