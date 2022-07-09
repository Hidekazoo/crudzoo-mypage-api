use crate::auth::claims::Claims;
use crate::configuration::get_configuration;
use chrono::Utc;
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;

pub enum ValidationError {
    TokenExpired,
    InvalidToken,
    DecodeError,
    Unexpected,
}

pub async fn validate_jwt_token(
    token: &str,
    // ) -> Pin<Box<impl Future<Output = Result<Claims, ValidationError>>>> {
) -> Result<Claims, ValidationError> {
    let config = get_configuration().unwrap();
    let header = match decode_header(token) {
        Ok(v) => v,
        Err(_) => {
            tracing::error!("Failed Validation");
            return Err(ValidationError::DecodeError);
        }
    };
    let kid = header.kid.ok_or(ValidationError::DecodeError)?;
    let token: String = token.to_string();
    let jwks: JwkSet = Client::new()
        .get(format!("{}", config.auth0_settings.domain))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let jwk = jwks.find(&kid).ok_or(ValidationError::DecodeError)?;
    match jwk.clone().algorithm {
        AlgorithmParameters::RSA(ref rsa) => {
            let mut validation = Validation::new(Algorithm::RS256);
            validation.set_audience(&[config.auth0_settings.audience]);
            validation.set_issuer(&[config.auth0_settings.issuer]);
            if let Ok(key) = DecodingKey::from_rsa_components(&rsa.n, &rsa.e) {
                return match decode::<Claims>(&token, &key, &validation) {
                    Ok(c) => {
                        let now = Utc::now().timestamp();
                        if c.claims.exp - now >= 3600 {
                            // 1時間
                            return Ok(c.claims);
                        }
                        Err(ValidationError::TokenExpired)
                    }
                    Err(e) => return Err(ValidationError::InvalidToken),
                };
            }
            Err(ValidationError::InvalidToken)
        }
        e => Err(ValidationError::Unexpected),
    }
}
