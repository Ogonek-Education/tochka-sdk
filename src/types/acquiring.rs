use crate::{PaymentMode, PaymentStatus, ReceiptClient, ReceiptItem, Supplier, TaxSystemCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentListQuery {
    /// Уникальный код клиента
    pub customer_code: String,
    /// Начало периода создания операций
    ///
    /// 2020-01-20
    pub from_date: Option<String>,
    /// Конец периода создания операций
    ///
    /// 2020-01-20
    pub to_date: Option<String>,
    /// Номер страницы
    pub page: Option<u32>,
    /// Количество записей на странице
    pub per_page: Option<u32>,
    /// Статус операции
    pub status: Option<PaymentStatus>,
}

impl PaymentListQuery {
    pub fn new(customer_code: impl Into<String>) -> Self {
        Self {
            customer_code: customer_code.into(),
            ..Default::default()
        }
    }

    pub fn page(mut self, v: u32) -> Self {
        self.page = Some(v);
        self
    }

    pub fn status(mut self, v: PaymentStatus) -> Self {
        self.status = Some(v);
        self
    }
}

#[derive(Deserialize, Validate, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentOperation {
    /// Уникальный код клиента
    #[validate(length(equal = 9))]
    pub customer_code: String,
    /// Система налогообложения
    pub tax_system_code: Option<TaxSystemCode>,
    /// Присутствует, если оплата произведена
    ///
    /// У Точки это два типа. По сути делают то же самое
    pub payment_type: Option<PaymentMode>,
    /// Идентификатор платежа в процессинге или СБП
    pub payment_id: Option<String>,
    /// Идентификатор транзакции в СБП
    ///
    /// Используется для возврата при оплате по СБП
    pub transation_id: Option<String>,
    /// Дата и время создания операции. Используется стандарт ISO8601
    pub created_at: DateTime<Utc>,
    /// Способ оплаты
    pub payment_mode: Option<Vec<PaymentMode>>,
    /// URL адрес, куда будет переправлен клиент после оплаты услуги
    pub redirect_url: Option<String>,
    /// URL адрес, куда будет переправлен клиент в случае неуспешной оплаты
    pub fail_redirect_url: Option<String>,
    /// Данные покупателя
    #[serde(rename = "Client")]
    pub client: Option<ReceiptClient>,
    /// Список товаров в заказе
    pub items: Option<Vec<ReceiptItem>>,
    /// Назначение платежа
    ///
    /// Отсутствует, если при создании платежа назначение не было указано
    pub purpose: Option<String>,
    /// Сумма платежа
    pub amount: u64,
    /// Статус платежа
    pub status: PaymentStatus,
    /// Идентификатор платежа
    pub operation_id: String,
    /// Ссылка на оплату
    pub payment_link: String,
    /// Идентификатор торговой точки в интернет-эквайринге
    pub merchant_id: Option<String>,
    /// Идентификатор покупателя
    ///
    /// uuid
    pub consumer_id: Option<String>,
    /// Список операций, связанных с платежом
    #[serde(rename = "Order")]
    pub order: Order,
    /// Данные поставщика
    #[serde(rename = "Supplier")]
    pub supplider: Option<Supplier>,
    /// Создать платёж с двухэтапной оплатой
    pub pre_authorization: Option<bool>,
    /// Дата и время оплаты
    ///
    /// В докуентации не сказано, что это ISO
    pub paid_at: Option<String>,
    /// Уникальный номер заказа
    #[validate(length(min = 1, max = 45))]
    pub payment_link_id: Option<String>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Order {
    /// Идентификатор платежа
    pub order_id: String,

    /// Тип операции
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Сумма операции
    pub amount: u32,

    /// Время операции
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug, EnumString, Display)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Refund,
    Approval,
    Authorized,
}
