use crate::auth::claims::Claims;
use crate::configuration::get_configuration;
use actix_web::http::Uri;
use actix_web::HttpRequest;
use chrono::Utc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

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
        _ => Err(false),
    }
}

pub enum ValidationError {
    TokenExpired,
    InvalidToken,
    Unexpected,
}

pub fn validate_jwt_token(token: &str) -> Result<Claims, ValidationError> {
    let config = get_configuration().unwrap();
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[config.auth0_settings.audience]);
    validation.set_issuer(&[Uri::builder()
        .scheme("https")
        .authority(config.auth0_settings.domain)
        .path_and_query("/")
        .build()
        .unwrap()]);

    if let Ok(key) =
        DecodingKey::from_rsa_components(&config.auth0_settings.rsa_n, &config.auth0_settings.rsa_e)
    {
        return match decode::<Claims>(token, &key, &validation) {
            Ok(c) => {
                let now = Utc::now().timestamp();
                if c.claims.exp - now >= 3600 {
                    // 1時間
                    return Ok(c.claims);
                }
                Err(ValidationError::TokenExpired)
            }
            Err(_) => Err(ValidationError::InvalidToken),
        };
    }
    Err(ValidationError::Unexpected)
}
