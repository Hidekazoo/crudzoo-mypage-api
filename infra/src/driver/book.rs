use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait(?Send)]
pub trait BookDriver {
    async fn add_book(&self, book_name: &str) -> Result<(), sqlx::Error>;
}

pub struct BookDriverImpl {
    pub pool: Arc<PgPool>,
}

#[async_trait(?Send)]
impl BookDriver for BookDriverImpl {
    async fn add_book(&self, book_name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO books (title, created_at, updated_at) VALUES ($1, $2, $3)
        "#,
            book_name,
            Utc::now(),
            Utc::now()
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }
}
