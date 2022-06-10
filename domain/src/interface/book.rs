use crate::errors::BookError;
use async_trait::async_trait;
use mockall::*;

pub struct AddBookParams {
    pub name: String,
}

#[automock]
#[async_trait(?Send)]
pub trait BookPort {
    async fn add_book(&self, params: &AddBookParams) -> Result<(), BookError>;
}

#[async_trait(?Send)]
pub trait BookUsecase {
    async fn add_book(
        &self,
        book_port: &dyn BookPort,
        params: &AddBookParams,
    ) -> Result<(), BookError>;
}
