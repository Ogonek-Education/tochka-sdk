#[derive(Debug, Clone, Copy)]
pub enum ApiVersion {
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
