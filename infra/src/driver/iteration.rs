use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Pool};

#[cfg_attr(test, faux::create)]
#[derive(Debug)]
pub struct IterationDriver {
  // pub pool: Arc<PgPool>,
  pub pool: Pool<Postgres>
}

#[derive(PartialEq, Eq, Debug)]
pub struct IterationStoreParam {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub hours: i32,
}

#[cfg_attr(test, faux::methods)]
impl IterationDriver {
    pub async fn store(&self, params: IterationStoreParam) -> anyhow::Result<()> {
      sqlx::query!(
        r#"
          INSERT INTO iteration (id, start_date, end_date, hours)
          VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        params.start_date,
        params.end_date,
        params.hours
      ).execute(&self.pool).await?;
      Ok(())
    }
}
