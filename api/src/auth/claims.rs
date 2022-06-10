use crate::auth::jwt::{get_token, validate_jwt_token, ValidationError};
use actix_web::dev::Payload;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{Error, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::{future::Future, pin::Pin};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    _permissions: Option<HashSet<String>>,
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let jwt = match get_token(&request) {
                Ok(v) => v,
                _ => {
                    return Err(ErrorUnauthorized("this route is protected"))},
            };
            match validate_jwt_token(&jwt).await {
                Ok(v) => Ok(v),
                Err(ValidationError::TokenExpired) => {
                    Err(ErrorUnauthorized("The access token expired."))
                }
                Err(ValidationError::InvalidToken) => {
                    Err(ErrorUnauthorized("The access token invalid."))
                }
                _ => Err(ErrorInternalServerError("internal server error.")),
            }
        })
    }
}
