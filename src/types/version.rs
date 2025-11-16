use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Версия API
pub enum ApiVersion {
    /// Других пока нет
    #[serde(rename = "v1.0")]
    V1_0,
}

impl ApiVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApiVersion::V1_0 => "v1.0",
        }
    }
}

impl Default for ApiVersion {
    fn default() -> Self {
        ApiVersion::V1_0
    }
}
