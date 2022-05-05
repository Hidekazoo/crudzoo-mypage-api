use crate::adaptor::UserRepository;
use crate::auth::claims::Claims;
use actix_web::{web, HttpResponse};
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
    _claims: Claims,
    pool: web::Data<PgPool>,
    form: web::Form<FormData>,
) -> HttpResponse {
    let connection_pool = pool.into_inner();
    let db = Database {
        pool: connection_pool,
    };
    let adaptor = UserRepository { db };
    let interactor = UserInteractor;
    interactor.create_user(&adaptor, &form.email).await.unwrap();
    HttpResponse::Ok().finish()
}
