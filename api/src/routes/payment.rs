use crate::adaptor::{PaymentRepository, PaymentTypeRepository};
use crate::auth::claims::Claims;
use actix_web::{web, HttpResponse};
use domain::errors::PaymentError;
use domain::interface::{
    AddPayment, FindPaymentParams, PaymentTypeUsecase, PaymentUsecase, StorePayment,
    StorePaymentData,
};
use domain::usecase;
use domain::usecase::PaymentInteractor;
use infra::{Database, PaymentDriverImpl, PaymentGateway};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize)]
struct UnauthorizedErrorResponse {
    message: String,
}

#[derive(serde::Serialize, Debug)]
struct PaymentType {
    id: i32,
    name: String,
}

#[derive(serde::Serialize)]
struct GetPaymentTypesResponse {
    payment_types: Vec<PaymentType>,
}

#[derive(serde::Serialize, Debug)]
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

#[tracing::instrument(
    name = "Get payment type", 
    skip(_claims, pool),
    fields(
        request_id = %Uuid::new_v4(),
    )    
)]
pub async fn get_payment_types(_claims: Claims, pool: web::Data<PgPool>) -> HttpResponse {
    let connection_pool = pool.into_inner();
    let db = Database {
        pool: connection_pool,
    };
    let adaptor = PaymentTypeRepository { db };
    let interactor = usecase::PaymentTypeInteractor;
    return match interactor.get_payment_types(&adaptor).await {
        Ok(v) => {
            let mut payment_types = Vec::new();
            for v in v.clone() {
                payment_types.push(PaymentType {
                    id: v.id.0,
                    name: v.name,
                })
            }
            tracing::info!("get payment type {:?}", payment_types);
            HttpResponse::Ok().json(GetPaymentTypesResponse { payment_types })
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::BadRequest().finish()
        }
    };
}

#[derive(serde::Deserialize, Debug)]
pub struct AddPaymentFormData {
    pub payment_type_id: i32,
    pub amount: i32,
}

#[tracing::instrument(
    name = "Add payment", 
    skip(_claims, pool),
    fields(
        request_id = %Uuid::new_v4(),
        payment_type_id=tracing::field::Empty,
        user_id=tracing::field::Empty,
        amount=tracing::field::Empty
    )    
)]
pub async fn add_payment(
    _claims: Claims,
    pool: web::Data<PgPool>,
    form: web::Json<AddPaymentFormData>,
) -> HttpResponse {
    let connection_pool = pool.into_inner();
    let adaptor = PaymentGateway {
        payment_driver: PaymentDriverImpl {
            pool: connection_pool,
        },
    };
    let interactor = PaymentInteractor;
    let params = StorePaymentData {
        payment_type_id: form.payment_type_id,
        amount: form.amount,
    };

    return match interactor.store(&adaptor, &params).await {
        Ok(_) => HttpResponse::Ok()
            .content_type("application/json")
            .json(AddPaymentResponse {
                payment_type_id: form.payment_type_id,
                amount: form.amount,
            }),
        Err(PaymentError::PaymentCreationError) => HttpResponse::InternalServerError().finish(),
        Err(error) => {
            tracing::warn!(
                error.cause_chain = ?error,
                "Failed to Add Payment."
            );
            HttpResponse::InternalServerError().finish()
        }
    };
}

// pub async fn find_payment(
//     _claims: Claims,
//     pool: web::Data<PgPool>,
//     path: web::Path<i32>,
// ) -> HttpResponse {
//     let connection_pool = pool.into_inner();
//     let db = Database {
//         pool: connection_pool,
//     };
//     let adaptor = PaymentRepository { db };
//     let user_id = path.into_inner();
//     let params = FindPaymentParams { user_id };
//     let interactor = usecase::PaymentInteractor;
//     let result = interactor.find_payment(&adaptor, &params).await.unwrap();
//     let mut payment = vec![];
//     for v in result.clone() {
//         payment.push(Payment {
//             id: v.id.0,
//             payment_type_id: v.payment_type_id.0,
//             amount: v.amount,
//             creation_date: v.creation_date,
//         })
//     }
//     HttpResponse::Ok().json(FindPaymentResponse { payment })
// }
