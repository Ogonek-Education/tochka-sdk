mod acquiring;
mod customer;
mod entities;
mod payment;
mod receipt;
mod service;
mod version;
pub use acquiring::*;
pub use customer::*;
pub use entities::*;
pub use payment::*;
pub use receipt::*;
pub use service::*;
pub use version::*;
mod external;
pub use external::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data<T> {
    /// Основная инфофрмация
    data: T,
    links: Link,
    meta: Meta,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[serde(rename_all = "PascalCase")]
pub struct PaginatedResponse<T> {
    pub links: PaginatedLink,
    pub meta: Meta,
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub total_pages: u64,
}
