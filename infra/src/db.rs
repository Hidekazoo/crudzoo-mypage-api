use sqlx::PgPool;
use std::sync::Arc;
use domain::interface::{DB};
use crate::payment::get_payment_types as db_get;

use domain::errors::paymentError;
use async_trait::async_trait;
use domain::interface::GetPaymentType;

#[derive(Clone)]
pub struct Database {
   pub pool: Arc<PgPool>
}

#[async_trait]
impl DB for Database {
    async fn get_payment_types(&self) -> Result<Vec<GetPaymentType>, paymentError> {
        return db_get(&self.pool).await;
    }
}