mod account;
mod balance;
mod consent;
mod entities;
mod payment;
mod receipt;
mod refund;
mod registry;
mod retailer;
mod service;
mod statements;
mod tax;
mod transactions;
mod version;
mod webhooks;

pub use account::*;
pub use balance::*;
pub use consent::*;
pub use entities::*;
pub use payment::*;
pub use receipt::*;
pub use refund::*;
pub use registry::*;
pub use retailer::*;
pub use service::*;
pub use statements::*;
pub use tax::*;
pub use transactions::*;
pub use version::*;
pub use webhooks::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data<T> {
    /// Основная инфофрмация
    pub data: T,
    pub links: Link,
    pub meta: Meta,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Link {
    #[serde(rename = "self")]
    pub this: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedLink {
    /// Self-link to the api
    #[serde(rename = "self")]
    pub this: String,

    pub first: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Wrapper for USUALLY a list endpoint
pub struct PaginatedResponse<T> {
    /// No idea why
    pub links: PaginatedLink,
    /// Pagination
    pub meta: Meta,
    /// Info
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A useless struct
pub struct Meta {
    /// Total pages
    pub total_pages: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A wrapper that adapts human logic to Tochka's api
pub struct PayloadWrapper<T> {
    /// The main info
    pub data: T,
}

impl<T> PayloadWrapper<T> {
    /// Does it what it says
    pub fn wrap(data: T) -> Self {
        Self { data }
    }
}

/// A single result field
///
/// Used in Delete enpoints as a return value
#[derive(Serialize, Deserialize, Debug)]
pub struct ResultBody {
    /// Whether the operation was successful
    pub result: bool,
}
