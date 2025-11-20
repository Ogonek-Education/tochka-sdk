use crate::{Client, Error};
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode};
use serde::{Deserialize, de::DeserializeOwned};

/// Токен Точки, использующийся в вебхуках
#[derive(Debug, Deserialize, Clone)]
pub struct Jwk {
    pub kty: String,
    pub n: String,
    pub e: String,
    pub kid: Option<String>,
    pub alg: Option<String>,
}

pub async fn fetch_jwk() -> Result<Jwk, Error> {
    let resp = reqwest::get("https://enter.tochka.com/doc/openapi/static/keys/public")
        .await
        .map_err(|e| Error::Config(e.to_string()))?
        .json::<Jwk>()
        .await
        .map_err(|e| Error::Config(e.to_string()))?;

    Ok(resp)
}
impl Client {
    pub fn decode_token<T>(&self, token: &str) -> jsonwebtoken::errors::Result<TokenData<T>>
    where
        T: DeserializeOwned,
    {
        let key = DecodingKey::from_rsa_components(&self.jwk.n, &self.jwk.e)?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = false;
        validation.validate_nbf = false;
        validation.required_spec_claims.clear();

        decode::<T>(token, &key, &validation)
    }
}
