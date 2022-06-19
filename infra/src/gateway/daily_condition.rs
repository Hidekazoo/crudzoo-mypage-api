use async_trait::async_trait;
use domain::{
    errors::DailyConditionError,
    interface::{DailyConditionPort, StoreDailyConditionParams},
};

use crate::{DailyConditionDriver, NewDailyCondition};

#[derive(Debug)]
pub struct DailyConditionGateway<T: DailyConditionDriver> {
    pub daily_condition_driver: T,
}

#[async_trait(?Send)]
impl<T: DailyConditionDriver> DailyConditionPort for DailyConditionGateway<T> {
    async fn store_daily_condition(
        &self,
        params: &StoreDailyConditionParams,
    ) -> Result<(), DailyConditionError> {
        let new_daily_condition = NewDailyCondition {
            weight: params.weight,
            sleep_time: params.sleep_time,
            mental_score: params.mental_score,
        };
        return match self
            .daily_condition_driver
            .store_daily_condition(&new_daily_condition)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(DailyConditionError::UnexpectedError),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MockDailyConditionDriver, StoreDailyConditionError};
    use domain::errors::DailyConditionError;

    #[tokio::test]
    async fn test_store_daily_condition_success() {
        let mut daily_condition_driver_mock = MockDailyConditionDriver::new();
        daily_condition_driver_mock
            .expect_store_daily_condition()
            .returning(|_| Ok(()));
        let params = StoreDailyConditionParams {
            weight: 100,
            mental_score: 82,
            sleep_time: 120,
        };
        let gateway = DailyConditionGateway {
            daily_condition_driver: daily_condition_driver_mock,
        };
        let result = gateway.store_daily_condition(&params).await;
        assert_eq!(result, Ok(()))
    }

    #[tokio::test]
    async fn test_store_daily_condition_fail() {
        let mut daily_condition_driver_mock = MockDailyConditionDriver::new();
        daily_condition_driver_mock
            .expect_store_daily_condition()
            .returning(|_| Err(StoreDailyConditionError(sqlx::Error::RowNotFound)));
        let params = StoreDailyConditionParams {
            weight: 100,
            mental_score: 82,
            sleep_time: 120,
        };
        let gateway = DailyConditionGateway {
            daily_condition_driver: daily_condition_driver_mock,
        };
        let result = gateway.store_daily_condition(&params).await;
        assert_eq!(result, Err(DailyConditionError::UnexpectedError))
    }
}
