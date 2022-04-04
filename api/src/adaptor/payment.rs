use async_trait::async_trait;
use domain::entity::{Payment, PaymentId, PaymentType, PaymentTypeId, UserId};
use domain::errors::PaymentError;
use domain::interface::{AddPayment, FindPaymentParams, PaymentDao, PaymentTypeDao, DB};

#[derive(Copy, Clone)]
pub struct PaymentTypeRepository<T: DB> {
    pub db: T,
}
#[derive(Copy, Clone)]
pub struct PaymentRepository<T: DB> {
    pub db: T,
}

#[async_trait(?Send)]
impl<T: DB> PaymentTypeDao for PaymentTypeRepository<T> {
    async fn get_payment_types(&self) -> Result<Vec<PaymentType>, PaymentError> {
        let result = self.db.get_payment_types().await?;
        let mut e = Vec::new();
        for i in result {
            e.push(PaymentType {
                id: PaymentTypeId(i.id),
                name: i.name,
            });
        }
        Ok(e)
    }
}

#[async_trait(?Send)]
impl<T: DB> PaymentDao for PaymentRepository<T> {
    async fn add_payment(&self, params: &AddPayment) -> Result<(), PaymentError> {
        match self
            .db
            .add_payment(&params.payment_type_id, &params.user_id, &params.amount)
            .await
        {
            Ok(_) => Ok(()),
            Err(PaymentError::PaymentCreationError) => Err(PaymentError::PaymentCreationError),
            _ => Err(PaymentError::UnexpectedError),
        }
    }
    async fn find_payment(&self, params: &FindPaymentParams) -> Result<Vec<Payment>, PaymentError> {
        let result = self.db.find_payment(&params.user_id).await?;
        let mut e = Vec::new();
        for i in result {
            e.push(Payment {
                id: PaymentId(i.id),
                user_id: UserId(params.user_id),
                payment_type_id: PaymentTypeId(i.payment_type_id),
                amount: i.amount,
            });
        }
        Ok(e)
    }
}
