use crate::auth::jwt::{get_token, validate_token};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use domain::interface::{AddBookParams, BookUsecase};
use domain::usecase::{BookInteractor};
use infra::{BookDriver};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(serde::Deserialize)]
pub struct FormData {
    pub name: String,
}

pub async fn add_book(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Json<FormData>,
) -> HttpResponse {
    let jwt = match get_token(&req) {
        Ok(v) => v,
        _ => {
            return HttpResponse::new(StatusCode::UNAUTHORIZED);
        }
    };
    match validate_token(&jwt) {
        Ok(v) => {
            if v {
                let connection_pool = pool.into_inner();
                let book_driver = BookDriver {
                  pool: connection_pool
                };
                let interactor = BookInteractor;
                let params = AddBookParams {
                    name: form.name.clone()
                };
                return match interactor.add_book(&book_driver, &params).await {
                    Ok(_) => HttpResponse::Ok().finish(),
                    _ => HttpResponse::BadRequest().json(ErrorResponse {
                        message: "Failed to create book data".to_string()
                    }),
                }
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                message: "invalid token".to_string(),
            })
        }
    }
    HttpResponse::Unauthorized().json(ErrorResponse {
        message: "invalid token".to_string(),
    })
}