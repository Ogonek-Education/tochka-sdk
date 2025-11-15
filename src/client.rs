use crate::{ApiVersion, Error, Scope, Service};
use std::time::Duration;

pub const PRODUCTION_BASE: &str = "https://enter.tochka.com/uapi/";
pub const SANDBOX_BASE: &str = "https://enter.tochka.com/sandbox/v2/";

pub enum Environment {
    Sandbox,
    Production,
}

impl Environment {
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Production => PRODUCTION_BASE,
            Environment::Sandbox => SANDBOX_BASE,
        }
    }
}

pub struct Client {
    client: reqwest::Client,
    env: Environment,
    token: String,
}

impl Client {
    pub fn new(env: Environment) -> Result<Self, Error> {
        let version = env!("CARGO_PKG_VERSION");
        let token = std::env::var("TOCHKA_TOKEN")?;

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .connect_timeout(Duration::from_secs(5))
            .user_agent(format!("tochka-rust-sdk/{version}"))
            .pool_idle_timeout(Some(Duration::from_secs(90)))
            .pool_max_idle_per_host(20)
            .build()
            .map_err(|e| Error::Config(e.to_string()))?;

        Ok(Self {
            client,
            env,
            token: token.into(),
        })
    }
}

impl Client {
    pub fn url(&self, service: Service, version: ApiVersion, path: &str) -> String {
        format!(
            "{}{}/{}/{}",
            self.env.base_url(),
            service.path(),
            version.as_str(),
            path.trim_start_matches('/')
        )
    }
}
