use crate::errors::DailyConditionError;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait(?Send)]
pub trait DailyConditionPort {
    async fn store_daily_condition(
        &self,
        params: &StoreDailyConditionParams,
    ) -> Result<(), DailyConditionError>;
}

pub struct StoreDailyConditionParams {
    pub weight: i32,
    pub sleep_time: i32,
    pub mental_score: i32,
}

#[async_trait(?Send)]
pub trait DailyConditionUsecase {
    async fn store_daily_condition(
        &self,
        daily_condition_port: &dyn DailyConditionPort,
        params: &StoreDailyConditionParams,
    ) -> Result<(), DailyConditionError>;
}
