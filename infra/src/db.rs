use crate::payment::get_payment_types as db_get;
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
        return db_get(&self.pool).await;
    }
    async fn create_user(&self, email: &str) -> Result<(), UserError> {
        match db_create(&self.pool, email).await {
            Ok(_) => Ok(()),
            _ => Err(UserError::UnexpectedError),
        }
    }
}
