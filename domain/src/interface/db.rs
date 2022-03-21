use async_trait::async_trait;
use crate::errors::paymentError;

pub struct GetPaymentType {
    pub id: i32,
    pub name: String
}

#[async_trait]
pub trait DB {
    async fn get_payment_types(&self) -> Result<Vec<GetPaymentType>, paymentError>;
}