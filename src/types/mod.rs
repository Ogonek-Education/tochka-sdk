mod acquiring;
mod auth;
mod entities;
mod payment;
mod receipt;
mod refund;
mod service;
mod version;

pub use acquiring::*;
pub use auth::*;
pub use entities::*;
pub use payment::*;
pub use receipt::*;
pub use refund::*;
use serde::Deserialize;
pub use service::*;
pub use version::*;

pub struct Data<T> {
    /// Основная инфофрмация
    data: T,
    links: Link,
    meta: Meta,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Link {
    #[serde(rename = "self")]
    pub this: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedLink {
    #[serde(rename = "self")]
    pub this: String,

    pub first: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse<T> {
    pub links: PaginatedLink,
    pub meta: Meta,
    pub data: Vec<T>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub total_pages: u64,
}
