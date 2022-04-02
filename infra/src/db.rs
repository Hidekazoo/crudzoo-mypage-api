use crate::payment::{add_payment as db_add_payment, get_payment_types as db_get_payment_type};
use domain::interface::DB;
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
    async fn add_payment(
        &self,
        payment_type_id: &i32,
        user_id: &i32,
        amount: &i32,
    ) -> Result<(), PaymentError> {
        match db_add_payment(&self.pool, payment_type_id, user_id, amount).await {
            Ok(_) => Ok(()),
            _ => Err(PaymentError::PaymentCreationError),
        }
    }
}
