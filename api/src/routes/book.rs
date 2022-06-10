use crate::auth::claims::Claims;
use actix_web::{web, HttpResponse};
use domain::interface::{AddBookParams, BookUsecase};
use domain::usecase::BookInteractor;
use infra::gateway::BookGateway;
use infra::driver::BookDriverImpl;
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
    _claims: Claims,
    // req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Json<FormData>,
) -> HttpResponse {
    let connection_pool = pool.into_inner();
    let book_port = BookGateway {
        book_driver: BookDriverImpl {
            pool: connection_pool
        }
    };
    let interactor = BookInteractor;
    let params = AddBookParams {
        name: form.name.clone(),
    };
    return match interactor.add_book(&book_port, &params).await {
        Ok(_) => HttpResponse::Ok().finish(),
        _ => HttpResponse::BadRequest().json(ErrorResponse {
            message: "Failed to create book data".to_string(),
        }),
    };
}
