use actix_web::http::Uri;
use actix_web::HttpRequest;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use chrono::Utc;
use crate::auth::claims::Claims;
use crate::configuration::get_configuration;

pub fn get_token(req: &HttpRequest) -> Result<String, bool> {
    let authorization = req
        .headers()
        .get("authorization")
        .map(|x| x.to_str().unwrap_or_default().to_string());
    let token = authorization.unwrap();
    let mut split_token = token.split(" ");
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

pub fn validate_token(token: &str) -> Result<bool, bool> {
    let config = get_configuration().unwrap();
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[config.audience]);
    validation.set_issuer(&[Uri::builder()
        .scheme("https")
        .authority(config.domain)
        .path_and_query("/")
        .build()
        .unwrap()]);

    let key = DecodingKey::from_rsa_components( &config.rsa_n, &config.rsa_e).unwrap();
    match decode::<Claims>(token, &key, &validation) {
        Ok(c) => {
            let now = Utc::now().timestamp();
            if c.claims.exp - now >= 3600 { // 1時間
                return Ok(true)
            }
            return Ok(false);
        }
        Err(error) => return Err(false),
    };
}
