use crate::adaptor::{PaymentRepository, PaymentTypeRepository};
use crate::auth::jwt::{get_token, validate_token};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use domain::errors::PaymentError;
use domain::interface::{AddPayment, FindPaymentParams, PaymentTypeUsecase, PaymentUsecase};
use domain::usecase;
use domain::usecase::PaymentInteractor;
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

#[derive(serde::Serialize)]
struct Payment {
    id: i32,
    payment_type_id: i32,
    amount: i32,
    creation_date: String,
}

#[derive(serde::Serialize)]
struct FindPaymentResponse {
    payment: Vec<Payment>,
}

#[derive(serde::Serialize)]
struct AddPaymentResponse {
    payment_type_id: i32,
    amount: i32,
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
                message: "invalid token".to_string(),
            })
        }
    }
    HttpResponse::Unauthorized().json(UnauthorizedErrorResponse {
        message: "invalid token".to_string(),
    })
}

#[derive(serde::Deserialize)]
pub struct AddPaymentFormData {
    pub payment_type_id: i32,
    pub user_id: i32,
    pub amount: i32,
}

pub async fn add_payment(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Json<AddPaymentFormData>,
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
                let adaptor = PaymentRepository { db };
                let interactor = PaymentInteractor;

                let params = AddPayment {
                    payment_type_id: form.payment_type_id,
                    user_id: form.user_id,
                    amount: form.amount,
                };
                //
                return match interactor.add_payment(&adaptor, &params).await {
                    Ok(_) => HttpResponse::Ok().content_type("application/json").json(
                        AddPaymentResponse {
                            payment_type_id: form.payment_type_id,
                            amount: form.amount,
                        },
                    ),
                    Err(PaymentError::PaymentCreationError) => {
                        HttpResponse::InternalServerError().finish()
                    }
                    Err(_) => HttpResponse::InternalServerError().finish(),
                };
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

pub async fn find_payment(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
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
                let adaptor = PaymentRepository { db };
                let user_id = path.into_inner();
                let params = FindPaymentParams { user_id };
                let interactor = usecase::PaymentInteractor;
                let result = interactor.find_payment(&adaptor, &params).await.unwrap();
                let mut payment = vec![];
                for v in result.clone() {
                    payment.push(Payment {
                        id: v.id.0,
                        payment_type_id: v.payment_type_id.0,
                        amount: v.amount,
                        creation_date: v.creation_date,
                    })
                }
                return HttpResponse::Ok().json(FindPaymentResponse { payment });
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
