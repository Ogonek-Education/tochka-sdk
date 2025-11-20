use crate::{ApiVersion, Error, Jwk, Service, jwt::fetch_jwk};
use std::time::Duration;

/// RU: Базовый URL продакшн-окружения Tochka API.  
/// EN: Base Tochka API production URL without version suffix.
pub const PRODUCTION_BASE: &str = "https://enter.tochka.com/uapi/";

/// RU: Базовый URL песочницы Tochka API.  
/// EN: Base Tochka API sandbox URL without version suffix.
pub const SANDBOX_BASE: &str = "https://enter.tochka.com/sandbox/v2/";

/// RU: Окружение, в котором выполняются запросы.  
/// EN: Endpoint environment selector.
#[derive(Clone, Debug, Default)]
pub enum Environment {
    /// RU: Песочница. EN: Sandbox endpoint.
    Sandbox,
    /// RU: Продакшн. EN: Production endpoint.
    #[default]
    Production,
}

impl From<&str> for Environment {
    fn from(s: &str) -> Self {
        match s {
            "PRODUCTION" => Self::Production,
            "SANDBOX" => Self::Sandbox,
            _ => Self::Sandbox,
        }
    }
}

impl From<String> for Environment {
    fn from(s: String) -> Self {
        Environment::from(s.as_str())
    }
}

impl Environment {
    /// RU: Вернуть базовый URL в зависимости от окружения.  
    /// EN: Return the base URL for the selected environment.
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Production => PRODUCTION_BASE,
            Environment::Sandbox => SANDBOX_BASE,
        }
    }
}

/// RU: Основной клиент SDK Tochka.  
/// EN: Main Tochka SDK client.
#[derive(Clone, Debug)]
pub struct Client {
    /// RU: HTTP-клиент reqwest. EN: Underlying reqwest client.
    pub(crate) client: reqwest::Client,
    /// RU: Идентификатор приложения (используется в вебхуках). EN: Application client ID (used in webhooks).
    pub(crate) client_id: Option<String>,
    /// RU: Уникальный идентификатор клиента, к которому подключен эквайринг
    pub customer_code: Option<String>,
    /// RU: Текущая среда (песочница или прод). EN: Current environment.
    env: Environment,
    /// Токен для расшифровки запросов вебхукам
    pub(crate) jwk: Jwk,
    /// RU: JWT/оAuth токен доступа. EN: Access token (JWT/OAuth).
    token: String,
}

impl Client {
    /// Создать клиента для указанного окружения.  
    pub async fn new() -> Result<Self, Error> {
        let version = env!("CARGO_PKG_VERSION");
        let env: Environment = std::env::var("TOCHKA_ENV")
            .unwrap_or(String::from("SANDBOX"))
            .into();
        let token = match env {
            Environment::Production => std::env::var("TOCHKA_TOKEN")?,
            Environment::Sandbox => "sandbox.jwt.token".to_string(),
        };

        let jwk = fetch_jwk().await?;

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
            jwk,
            client_id: None,
            customer_code: None,
        })
    }

    /// RU: Получить customer_code для Business-аккаунта.  
    ///
    /// Если задана переменная окружения `CUSTOMER_CODE`, будет использована она. Иначе SDK
    /// выполнит `get_accounts_list`, отфильтрует Business-аккаунты и:
    /// - если найден один — вернёт его;
    /// - если найдено несколько — вернёт ошибку конфигурации и предложит установить `CUSTOMER_CODE`.
    pub async fn with_client_code(mut self) -> Result<Self, Error> {
        self.customer_code = Some(self.resolve_business_customer_code().await?);

        Ok(self)
    }

    /// Добоавить client id в клиент. Нужен для вебхуков
    pub fn with_client_id(mut self) -> Result<Self, Error> {
        let client_id = std::env::var("TOCHKA_CLIENT_ID")?;
        self.client_id = Some(client_id);

        Ok(self)
    }
}

impl Client {
    /// RU: Собрать полный URL для сервиса/версии/пути.  
    /// EN: Build a fully-qualified URL for the given service, version and path.
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
    /// RU: Отправить запрос: добавить авторизацию, проверить HTTP-статусы и десериализовать тело.  
    /// EN: Send a request with auth, map HTTP errors, and deserialize the body.
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
