use crate::entity::{Payment, PaymentType};
use crate::errors::PaymentError;
use crate::interface::PaymentTypeUsecase;
use crate::interface::{AddPayment, FindPaymentParams, PaymentDao, PaymentTypeDao, PaymentUsecase};
use async_trait::async_trait;

#[derive(Debug)]
pub struct PaymentTypeInteractor;

#[derive(Debug)]
pub struct PaymentInteractor;

#[async_trait(?Send)]
impl PaymentTypeUsecase for PaymentTypeInteractor {
    async fn get_payment_types(
        &self,
        payment_type_dao: &dyn PaymentTypeDao,
    ) -> Result<Vec<PaymentType>, PaymentError> {
        let result = payment_type_dao.get_payment_types().await;
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }
}

#[async_trait(?Send)]
impl PaymentUsecase for PaymentInteractor {
    async fn add_payment(
        &self,
        payment_dao: &dyn PaymentDao,
        params: &AddPayment,
    ) -> Result<(), PaymentError> {
        payment_dao.add_payment(params).await?;
        Ok(())
    }
    async fn find_payment(
        &self,
        payment_dao: &dyn PaymentDao,
        params: &FindPaymentParams,
    ) -> Result<Vec<Payment>, PaymentError> {
        match payment_dao.find_payment(params).await {
            Ok(v) => Ok(v),
            _ => Err(PaymentError::FindPaymentError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{Payment, PaymentTypeId};
    use crate::interface::{AddPayment, MockPaymentDao, MockPaymentTypeDao};

    #[tokio::test]
    async fn test_get_payment_types_success() {
        let mut mock = MockPaymentTypeDao::new();
        let mut v = Vec::new();
        let payment_types = PaymentType {
            id: PaymentTypeId(1),
            name: "test".to_string(),
        };
        v.push(payment_types);
        mock.expect_get_payment_types().return_const(Ok(v.clone()));
        let interactor = PaymentTypeInteractor;
        let result = interactor.get_payment_types(&mock).await.unwrap();

        assert_eq!(result, v);
    }

    #[tokio::test]
    async fn test_get_payment_types_error() {
        let mut mock = MockPaymentTypeDao::new();
        mock.expect_get_payment_types()
            .return_const(Err(PaymentError::UnexpectedError));
        let interactor = PaymentTypeInteractor;
        let result = interactor.get_payment_types(&mock).await;

        assert_eq!(result, Err(PaymentError::UnexpectedError));
    }

    #[tokio::test]
    async fn test_add_payment_success() {
        let mut mock = MockPaymentDao::new();
        mock.expect_add_payment().return_const(Ok(()));
        let payment = AddPayment {
            payment_type_id: 2,
            user_id: 1,
            amount: 900,
        };
        let interactor = PaymentInteractor;
        let result = interactor.add_payment(&mock, &payment).await;
        assert_eq!(result.ok(), Some(()));
    }

    #[tokio::test]
    async fn test_create_user_fail() {
        let mut mock = MockPaymentDao::new();
        mock.expect_add_payment()
            .return_const(Err(PaymentError::UnexpectedError));

        let payment = AddPayment {
            payment_type_id: 2,
            user_id: 1,
            amount: 900,
        };

        let interactor = PaymentInteractor;
        let result = interactor.add_payment(&mock, &payment).await;
        assert_eq!(result.err(), Some(PaymentError::UnexpectedError));
    }
}
