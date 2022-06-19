use crate::{
    errors::DailyConditionError,
    interface::{DailyConditionPort, DailyConditionUsecase, StoreDailyConditionParams},
};
use async_trait::async_trait;

#[derive(Debug)]
pub struct DailyConditionInteractor;

#[async_trait(?Send)]
impl DailyConditionUsecase for DailyConditionInteractor {
    async fn store_daily_condition(
        &self,
        daily_condition_dao: &dyn DailyConditionPort,
        params: &StoreDailyConditionParams,
    ) -> Result<(), DailyConditionError> {
        daily_condition_dao.store_daily_condition(params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::{MockDailyConditionPort, StoreDailyConditionParams};
    #[tokio::test]
    async fn test_store_daily_condition() {
        let mut daily_condition_port_mock = MockDailyConditionPort::new();
        daily_condition_port_mock
            .expect_store_daily_condition()
            .return_const(Ok(()));
        let params = StoreDailyConditionParams {
            weight: 60,
            mental_score: 35,
            sleep_time: 100,
        };
        let interactor = DailyConditionInteractor;
        let result = interactor
            .store_daily_condition(&daily_condition_port_mock, &params)
            .await;

        assert_eq!(result, Ok(()))
    }
}
