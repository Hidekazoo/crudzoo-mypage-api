use async_trait::async_trait;
use axum::{Extension, response::IntoResponse};
use domain::{interface::{PaymentTypeDao, DB}, errors::PaymentError, entity::{PaymentType as A, PaymentTypeId as B}};
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::auth::claims::Claims;



#[derive(Copy, Clone)]
pub struct PaymentTypeRepository<T: DB> {
    pub db: T,
}


#[async_trait(?Send)]
impl<T: DB> PaymentTypeDao for PaymentTypeRepository<T> {
    async fn get_payment_types(&self) -> Result<Vec<A>, PaymentError> {
        let result = self.db.get_payment_types().await?;
        let mut e = Vec::new();
        for i in result {
            e.push(A {
                id: B(i.id),
                name: i.name,
            });
        }
        Ok(e)
    }
}



#[derive(serde::Serialize, Debug)]
struct PaymentType {
    id: i32,
    name: String,
}


#[derive(serde::Serialize)]
pub struct GetPaymentTypesResponse {
    payment_types: Vec<PaymentType>,
}

pub async fn get_payment_types(_claims: Claims, Extension(_pool): Extension<PgPool>) -> impl IntoResponse {
    return StatusCode::OK
//   let db = Database {
//       pool: Arc::new(pool),
//   };

//   let adaptor = PaymentTypeRepository { db };
//   let interactor = usecase::PaymentTypeInteractor;
//   return match interactor.get_payment_types(&adaptor).await {
    //   Ok(v) => {
    //       let mut payment_types = Vec::new();
    //       for v in v.clone() {
    //           payment_types.push(PaymentType {
    //               id: v.id.0,
    //               name: v.name,
    //           })
    //       }
    //     //   tracing::info!("get payment type {:?}", payment_types);
    //     //  Ok(Json(GetPaymentTypesResponse { payment_types }))
    //     // return StatusCode::OK
    //   },
    //   Err(e) => {
    //     //   tracing::error!("Failed to execute query: {:?}", e);
    //     //   Err(StatusCode::INTERNAL_SERVER_ERROR)
    //     // return StatusCode::INTERNAL_SERVER_ERROR
        
    //     //   HttpResponse::BadRequest().finish()
    //     _ => (StatusCode::ACCEPTED, format!("aaaa"))
    //   };
}