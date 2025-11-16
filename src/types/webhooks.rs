use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

#[derive(Serialize, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub webhooks_list: Vec<WebhookType>,
    #[validate(length(max = 2083))]
    pub url: String,
}

#[derive(Serialize, Debug, Deserialize, EnumString, Display)]
#[serde(rename_all = "camelCase")]
pub enum WebhookType {
    IncomingPayment,
    OutgoingPayment,
    IncomingSbpPayment,
    AcquiringInternetPayment,
    IncomingSbpB2BPayment,
}
