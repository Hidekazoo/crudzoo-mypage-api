use crate::payment::{
    get_payment_types as db_get_payment_type,
};
use domain::interface::{FindPayment, DB};
use sqlx::PgPool;
use std::sync::Arc;

use crate::user::create_user as db_create;
use async_trait::async_trait;
use domain::errors::{PaymentError, UserError};
use domain::interface::GetPaymentType;

#[derive(Clone)]
pub struct Database {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl DB for Database {
    async fn get_payment_types(&self) -> Result<Vec<GetPaymentType>, PaymentError> {
        return db_get_payment_type(&self.pool).await;
    }
    async fn create_user(&self, email: &str) -> Result<(), UserError> {
        match db_create(&self.pool, email).await {
            Ok(_) => Ok(()),
            _ => Err(UserError::UnexpectedError),
        }
    }
    // async fn find_payment(&self, user_id: &i32) -> Result<Vec<FindPayment>, PaymentError> {
    //     match db_find_payment(&self.pool, user_id).await {
    //         Ok(v) => {
    //             let mut result: Vec<FindPayment> = vec![];
    //             for i in v {
    //                 let item = FindPayment {
    //                     id: i.id,
    //                     payment_type_id: i.payment_type_id,
    //                     amount: i.amount,
    //                     creation_date: i.creation_date,
    //                 };
    //                 result.push(item);
    //             }
    //             Ok(result)
    //         }
    //         _ => Err(PaymentError::FindPaymentError),
    //     }
    // }
}
