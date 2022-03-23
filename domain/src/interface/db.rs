use crate::errors::{PaymentError, UserError};
use async_trait::async_trait;

pub struct GetPaymentType {
    pub id: i32,
    pub name: String,
}

#[async_trait]
pub trait DB {
    async fn get_payment_types(&self) -> Result<Vec<GetPaymentType>, PaymentError>;
    async fn create_user(&self, email: &str) -> Result<(), UserError>;
    async fn add_payment(&self, payment_type_id: &i32, user_id: &i32, amount: &i32) -> Result<(), PaymentError>;
}
