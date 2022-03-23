use async_trait::async_trait;
use domain::entity::{PaymentType, PaymentTypeId};
use domain::errors::PaymentError;
use domain::errors::PaymentError::PaymentCreationError;
use domain::interface::{PaymentTypeDao, PaymentDao, DB, AddPayment};

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
        match self.db.add_payment(&params.payment_type_id, &params.user_id, &params.amount ).await {
            Ok(_) => Ok(()),
            Err(PaymentError::PaymentCreationError) => Err(PaymentError::PaymentCreationError),
            _ => Err(PaymentError::UnexpectedError)
        }
    }
}