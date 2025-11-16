use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Полностью ли оплачен
#[derive(Serialize, Debug, Deserialize, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
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
#[derive(Deserialize, Serialize, Debug, EnumString, Display, PartialEq)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PaymentMode {
    Sbp,
    Card,
    Tinkoff,
    Dolyame,
}

/// Признак предмета платежа
#[derive(Deserialize, Serialize, Debug, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PaymentObject {
    Goods,
    Service,
    Work,
}
use crate::{ReceiptClient, ReceiptItem, Supplier, TaxSystemCode};
use chrono::{DateTime, NaiveDate, Utc};
use validator::Validate;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentListQuery {
    /// Уникальный код клиента
    pub customer_code: String,
    /// Начало периода создания операций
    ///
    /// 2020-01-20
    pub from_date: Option<NaiveDate>,
    /// Конец периода создания операций
    ///
    /// 2020-01-20
    pub to_date: Option<NaiveDate>,
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

    pub fn from_date(mut self, fd: impl Into<NaiveDate>) -> Self {
        self.from_date = Some(fd.into());
        self
    }

    pub fn to_date(mut self, td: impl Into<NaiveDate>) -> Self {
        self.to_date = Some(td.into());
        self
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
    ///
    /// Его нет в ответах POST запросов. Всегда есть в GET
    #[validate(length(equal = 9))]
    pub customer_code: Option<String>,
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
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<String>,
    /// Дата и время создания операции. Используется стандарт ISO8601
    ///
    /// Нет в POST, есть в GET
    pub created_at: Option<DateTime<Utc>>,
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
    pub amount: f64,
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
    pub order: Option<Vec<Order>>,
    /// Данные поставщика
    #[serde(rename = "Supplier")]
    pub supplier: Option<Supplier>,
    /// Создать платёж с двухэтапной оплатой
    pub pre_authorization: Option<bool>,
    /// Дата и время оплаты
    ///
    /// В докуентации не сказано, что это ISO
    pub paid_at: Option<String>,
    /// Уникальный номер заказа
    #[validate(length(min = 1, max = 45))]
    pub payment_link_id: Option<String>,
    // Create implementation
    pub save_card: Option<bool>,
    pub ttl: Option<i64>,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Идентификатор платежа
    pub order_id: String,

    /// Тип операции
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Сумма операции
    pub amount: f64,

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentPayload {
    pub amount: f64,
    pub consumer_id: Option<String>,
    pub customer_code: String,
    pub fail_redirect_url: Option<String>,
    pub merchant_id: Option<String>,
    pub payment_link_id: Option<String>,
    pub payment_mode: Vec<PaymentMode>,
    pub pre_authorization: Option<bool>,
    pub purpose: String,
    pub redirect_url: Option<String>,
    pub save_card: Option<bool>,
    pub ttl: Option<i64>,
}

impl CreatePaymentPayload {
    pub fn new(amount: f64, customer_code: impl Into<String>, purpose: impl Into<String>) -> Self {
        Self {
            amount,
            customer_code: customer_code.into(),
            purpose: purpose.into(),
            consumer_id: None,
            fail_redirect_url: None,
            merchant_id: None,
            payment_link_id: None,
            payment_mode: Vec::new(),
            pre_authorization: None,
            redirect_url: None,
            save_card: None,
            ttl: None,
        }
    }

    pub fn fail_redirect_url(mut self, fail_redirect_url: impl Into<String>) -> Self {
        self.fail_redirect_url = Some(fail_redirect_url.into());
        self
    }

    pub fn redirect_url(mut self, redirect_url: impl Into<String>) -> Self {
        self.redirect_url = Some(redirect_url.into());
        self
    }

    pub fn save_card(mut self, save_card: bool) -> Self {
        self.save_card = Some(save_card);
        self
    }

    pub fn consumer_id(mut self, id: impl Into<String>) -> Self {
        self.consumer_id = Some(id.into());
        self
    }

    pub fn merchant_id(mut self, id: impl Into<String>) -> Self {
        self.merchant_id = Some(id.into());
        self
    }

    pub fn payment_mode(mut self, mode: PaymentMode) -> Self {
        self.payment_mode.push(mode);
        self
    }

    pub fn payment_link_id(mut self, id: impl Into<String>) -> Self {
        self.payment_link_id = Some(id.into());
        self
    }

    pub fn pre_authorization(mut self, pa: bool) -> Self {
        self.pre_authorization = Some(pa);
        self
    }

    pub fn ttl(mut self, value: i64) -> Self {
        self.ttl = Some(value);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentPageData {
    pub operation: Vec<PaymentOperation>,
}
