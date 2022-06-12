use crate::driver::BookDriver;
use async_trait::async_trait;
use domain::errors::BookError;
use domain::interface::{AddBookParams, BookPort};

#[derive(Clone)]
pub struct BookGateway<T: BookDriver> {
    pub book_driver: T,
}

#[async_trait(?Send)]
impl<T: BookDriver> BookPort for BookGateway<T> {
    async fn add_book(&self, params: &AddBookParams) -> Result<(), BookError> {
        match self.book_driver.add_book(&params.name).await {
            Ok(_) => Ok(()),
            Err(_) => Err(BookError::UnexpectedError),
        }
    }
}
