use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use mockall::automock;
use sqlx::PgPool;

#[automock]
#[async_trait(?Send)]
pub trait PaymentDriver {
    async fn store(&self, payment_type_id: &i32, amount: &i32) -> Result<(), sqlx::Error>;
}

pub struct PaymentDriverImpl {
    pub pool: Arc<PgPool>,
}

#[async_trait(?Send)]
impl PaymentDriver for PaymentDriverImpl {
    async fn store(&self, payment_type_id: &i32, amount: &i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
      r#"
        INSERT INTO payment (payment_type_id, amount, created_at, updated_at) VALUES ($1, $2, $3, $4)
      "#,
      payment_type_id,
      amount,
      Utc::now(),
      Utc::now()
    ).execute(&*self.pool).await?;
        Ok(())
    }
}
