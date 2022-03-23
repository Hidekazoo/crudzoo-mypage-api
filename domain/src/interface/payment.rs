use crate::entity::PaymentType;
use crate::errors::PaymentError;
use async_trait::async_trait;
use mockall::*;

#[automock]
#[async_trait(?Send)]
pub trait PaymentTypeDao {
    async fn get_payment_types(&self) -> Result<Vec<PaymentType>, PaymentError>;
}

#[async_trait(?Send)]
pub trait PaymentTypeUsecase {
    async fn get_payment_types(
        &self,
        payment_type_dao: &dyn PaymentTypeDao,
    ) -> Result<Vec<PaymentType>, PaymentError>;
}
