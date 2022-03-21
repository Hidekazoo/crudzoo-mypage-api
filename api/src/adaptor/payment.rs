
use domain::entity::{PaymentType, PaymentTypeId};
use domain::interface::{PaymentTypeDao, DB};
use domain::errors::paymentError;
use async_trait::async_trait;

#[derive(Copy, Clone)]
pub struct PaymentTypeRepository<T: DB> {
    pub db: T
}

#[async_trait(?Send)]
impl<T: DB> PaymentTypeDao for PaymentTypeRepository<T> {
    async fn get_payment_types(&self) -> Result<Vec<PaymentType>, paymentError> {
        let result = self.db.get_payment_types().await?;
        let mut e = Vec::new();
        for i in result {
            e.push(PaymentType {
                id: PaymentTypeId(i.id),
                name: i.name
            });
        }
        Ok(e)
    }
}