use crate::{Error, Scope};
use std::time::Duration;

pub const PRODUCTION_ENDPOINT: &str = "https://enter.T.com/uapi/open-banking";
pub const SANDBOX_ENDPOINT: &str = "https://enter.T.com/sandbox/v2/open-banking";

pub struct Client {
    client: reqwest::Client,
    base_url: String,
    token: String,
    scopes: Vec<Scope>,
}

impl Client {
    pub fn new(
        base_url: impl Into<String>,
        token: impl Into<String>,
        scopes: Vec<Scope>,
    ) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .connect_timeout(Duration::from_secs(5))
            .user_agent("tochka-rust-sdk/0.1")
            .pool_idle_timeout(Some(Duration::from_secs(90)))
            .pool_max_idle_per_host(20)
            .build()
            .map_err(|e| Error::Config(e.to_string()))?;

        Ok(Self {
            client,
            base_url: base_url.into(),
            token: token.into(),
            scopes: scopes,
        })
    }
}

impl Client {
    fn requires(&self, needed: Scope) -> Result<(), Error> {
        if self.scopes.contains(&needed) {
            Ok(())
        } else {
            Err(Error::MissingScope(needed.to_string()))
        }
    }
}
