use crate::{ApiVersion, Error, Service};
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
    pub client: reqwest::Client,
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

impl Client {
    /// The hook that processes errors, outputs deserialization issues, and puts on auth bearers
    pub async fn send<T>(&self, req: reqwest::RequestBuilder) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let resp = req.bearer_auth(&self.token).send().await.map_err(|e| {
            if e.is_timeout() {
                Error::Timeout
            } else {
                Error::Network(e.without_url().to_string())
            }
        })?;

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default(); // always capture raw JSON

        match status {
            reqwest::StatusCode::UNAUTHORIZED => return Err(Error::Unauthorized),
            reqwest::StatusCode::FORBIDDEN => return Err(Error::Forbidden),
            reqwest::StatusCode::NOT_FOUND => return Err(Error::NotFound),
            reqwest::StatusCode::TOO_MANY_REQUESTS => return Err(Error::TooManyRequests),
            code if code.is_server_error() => {
                return Err(Error::Server(body));
            }
            _ => {}
        }

        if !status.is_success() {
            return Err(Error::Api(body));
        }

        // ------- Enhanced Deserialization --------
        let mut deserializer = serde_json::Deserializer::from_str(&body);

        match serde_path_to_error::deserialize::<_, T>(&mut deserializer) {
            Ok(result) => Ok(result),
            Err(err) => {
                let path = err.path().to_string();
                let inner = err.into_inner();

                Err(Error::Deserialize {
                    message: inner.to_string(),
                    path,
                    raw: body,
                })
            }
        }
    }
}
