use crate::entity::PaymentType;
use crate::errors::PaymentError;
use crate::interface::PaymentTypeDao;
use crate::interface::PaymentTypeUsecase;
use async_trait::async_trait;

#[derive(Debug)]
pub struct PaymentTypeInteractor;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::PaymentTypeId;
    use crate::interface::MockPaymentTypeDao;

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
}
