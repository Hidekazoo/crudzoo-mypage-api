use sqlx::PgPool;
use std::sync::Arc;
use domain::errors::BookError;
use domain::interface::{AddBookParams, BookDao};
use chrono::Utc;
use async_trait::async_trait;

#[derive(Clone)]
pub struct BookDriver {
    pub pool: Arc<PgPool>
}

#[async_trait(?Send)]
impl BookDao for BookDriver {
    async fn add_book(&self, params: &AddBookParams) -> Result<(), BookError> {
        match add_book_data(&self.pool, &params.name).await {
            Ok(_) => Ok(()),
            Err(_) => Err(BookError::UnexpectedError)
        }
    }
}

pub async fn add_book_data(
    pool: &PgPool,
    book_name: &str
) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"
        INSERT INTO books (title, created_at, updated_at) VALUES ($1, $2, $3)
    "#, book_name, Utc::now(), Utc::now()).execute(pool).await?;
    Ok(())
}