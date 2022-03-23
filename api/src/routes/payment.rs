use crate::adaptor::PaymentTypeRepository;
use crate::auth::jwt::{get_token, validate_token};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use domain::interface::PaymentTypeUsecase;
use domain::usecase;
use infra::Database;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct UnauthorizedErrorResponse {
    message: String,
}

#[derive(serde::Serialize)]
struct PaymentType {
    id: i32,
    name: String,
}

#[derive(serde::Serialize)]
struct GetPaymentTypesResponse {
    payment_types: Vec<PaymentType>,
}

pub async fn get_payment_types(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
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
                let adaptor = PaymentTypeRepository { db };
                let interactor = usecase::PaymentTypeInteractor;
                let result = interactor.get_payment_types(&adaptor).await.unwrap();
                let mut payment_types = Vec::new();
                for v in result.clone() {
                    payment_types.push(PaymentType {
                        id: v.id.0,
                        name: v.name,
                    })
                }
                return HttpResponse::Ok().json(GetPaymentTypesResponse { payment_types });
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().json(UnauthorizedErrorResponse {
                message: "invalid token23".to_string(),
            })
        }
    }
    HttpResponse::Unauthorized().json(UnauthorizedErrorResponse {
        message: "invalid token22".to_string(),
    })
}
