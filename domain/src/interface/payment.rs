use crate::entity::{PaymentType, Payment};
use crate::errors::PaymentError;
use async_trait::async_trait;
use mockall::*;

#[automock]
#[async_trait(?Send)]
pub trait PaymentTypeDao {
    async fn get_payment_types(&self) -> Result<Vec<PaymentType>, PaymentError>;
}

#[automock]
#[async_trait(?Send)]
pub trait PaymentDao {
    async fn add_payment(&self, params: &AddPayment) -> Result<(), PaymentError>;
}

#[async_trait(?Send)]
pub trait PaymentTypeUsecase {
    async fn get_payment_types(
        &self,
        payment_type_dao: &dyn PaymentTypeDao,
    ) -> Result<Vec<PaymentType>, PaymentError>;
}

pub struct AddPayment {
    pub payment_type_id: i32,
    pub user_id: i32,
    pub amount: i32
}

#[async_trait(?Send)]
pub trait PaymentUsecase {
    async fn add_payment(
        &self,
        payment_dao: &dyn PaymentDao,
        params: &AddPayment
    ) -> Result<(), PaymentError>;
}
