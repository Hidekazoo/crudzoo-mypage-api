use crate::auth::claims::Claims;
use crate::configuration::get_configuration;
use actix_web::http::Uri;
use actix_web::HttpRequest;
use chrono::Utc;
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;

pub fn get_token(req: &HttpRequest) -> Result<String, bool> {
    let authorization = req
        .headers()
        .get("authorization")
        .map(|x| x.to_str().unwrap_or_default().to_string());
    match authorization {
        Some(v) => {
            let mut split_token = v.split(" ");

            match split_token.next() {
                None => return Err(false),
                Some(schema_type) => {
                    if schema_type != "Bearer" {
                        return Err(false);
                    }
                }
            };

            let jwt = match split_token.next() {
                None => return Err(false),
                Some(jwt) => jwt,
            };

            Ok(jwt.to_string())
        }
        _ => {
            tracing::error!("Failed to get token");
            Err(false)
        }
    }
}

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
