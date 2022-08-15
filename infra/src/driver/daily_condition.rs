use async_trait::async_trait;
use chrono::Utc;
use mockall::automock;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NewDailyCondition {
    pub weight: i32,
    pub sleep_time: i32,
    pub mental_score: i32,
}

// #[derive(Debug, Error, Clone)]
// pub enum StoreDailyConditionError {
//     #[error("")]
//     UnexpectedError,
// }

#[automock]
#[async_trait(?Send)]
pub trait DailyConditionDriver {
    async fn store_daily_condition(
        &self,
        params: &NewDailyCondition,
    ) -> Result<(), StoreDailyConditionError>;
}

pub struct DailyConditionDriverImpl {
    pub pool: Arc<PgPool>,
}

pub struct StoreDailyConditionError(pub sqlx::Error);

#[async_trait(?Send)]
impl DailyConditionDriver for DailyConditionDriverImpl {
    async fn store_daily_condition(
        &self,
        params: &NewDailyCondition,
    ) -> Result<(), StoreDailyConditionError> {
        sqlx::query!(
            r#"
      INSERT INTO daily_condition (weight, sleep_time, mental_score, created_at, updated_at) 
      VALUES ($1, $2, $3, $4, $5)
      "#,
            params.weight,
            params.sleep_time,
            params.mental_score,
            Utc::now(),
            Utc::now()
        )
        .execute(&*self.pool)
        .await
        .map_err(StoreDailyConditionError)?;
        Ok(())
    }
}
