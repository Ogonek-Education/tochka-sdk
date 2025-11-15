#[derive(Debug, Clone, Copy)]
/// Версия API
pub enum ApiVersion {
    /// Других пока нет
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
