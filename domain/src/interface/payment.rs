use crate::errors::paymentError;
use crate::entity::PaymentType;
use async_trait::async_trait;
use mockall::*;

#[automock]
#[async_trait(?Send)]
pub trait PaymentTypeDao {
    async fn get_payment_types(&self) -> Result<Vec<PaymentType>, paymentError>;
}

#[async_trait(?Send)]
pub trait PaymentTypeUsecase {
    async fn get_payment_types(&self, payment_types_repository: &dyn PaymentTypeDao) ->  Result<Vec<PaymentType>, paymentError>;
}