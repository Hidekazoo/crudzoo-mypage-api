use crate::adaptor::UserRepository;
use crate::auth::jwt::{get_token, validate_token};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use domain::interface::UserUsecase;
use domain::usecase::UserInteractor;
use infra::Database;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct UnauthorizedErrorResponse {
    message: String,
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
}

pub async fn create_user(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Form<FormData>,
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
                let db = Database {
                    pool: connection_pool,
                };
                let adaptor = UserRepository { db };
                let interactor = UserInteractor;
                interactor.create_user(&adaptor, &form.email).await.unwrap();
                return HttpResponse::Ok().finish();
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().json(UnauthorizedErrorResponse {
                message: "invalid token".to_string(),
            })
        }
    }
    HttpResponse::Unauthorized().json(UnauthorizedErrorResponse {
        message: "invalid token".to_string(),
    })
}
