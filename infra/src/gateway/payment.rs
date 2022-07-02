use async_trait::async_trait;
use domain::{interface::{StorePayment, StorePaymentData}, errors::PaymentError};

use crate::PaymentDriver;

#[derive(Debug)]
pub struct PaymentGateway<T: PaymentDriver> {
  pub payment_driver: T
}

#[async_trait(?Send)]
impl<T: PaymentDriver> StorePayment for PaymentGateway<T> {
  async fn store(&self, data: &StorePaymentData) -> Result<(), PaymentError> {
    return match self.payment_driver.store(&data.payment_type_id, &data.amount).await {
      Ok(_) => Ok(()),
      _ => Err(PaymentError::UnexpectedError)
    }
  }
}


#[cfg(test)]
mod tests {

  use super::*;
  use crate::MockPaymentDriver;

  #[tokio::test]
  async fn test_store_success() {
    let mut payment_driver = MockPaymentDriver::new();
    payment_driver.expect_store().returning(|_,_| Ok(()));

    let data = StorePaymentData {
      payment_type_id: 1,
      amount: 1000
    };

    let target = PaymentGateway {
      payment_driver
    };
    assert_eq!(target.store(&data).await, Ok(()))
  }

  #[tokio::test]
  async fn test_store_failure() {
    let mut payment_driver = MockPaymentDriver::new();
    payment_driver.expect_store().returning(|_,_| Err(sqlx::Error::RowNotFound));

    let data = StorePaymentData {
      payment_type_id: 1,
      amount: 1000
    };
    let target = PaymentGateway {
      payment_driver
    };
    let result = target.store(&data).await;
    assert_eq!(result, Err(PaymentError::UnexpectedError))
  }
}