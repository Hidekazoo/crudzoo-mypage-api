use crate::errors::BookError;
use crate::interface::{AddBookParams, BookPort, BookUsecase};
use async_trait::async_trait;

#[derive(Debug)]
pub struct BookInteractor;

#[async_trait(?Send)]
impl BookUsecase for BookInteractor {
    async fn add_book(
        &self,
        book_dao: &dyn BookPort,
        params: &AddBookParams,
    ) -> Result<(), BookError> {
        book_dao.add_book(params).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::{AddBookParams, MockBookPort};

    #[tokio::test]
    async fn test_add_book_success() {
        let mut mock = MockBookPort::new();
        mock.expect_add_book().return_const(Ok(()));
        let params = AddBookParams {
            name: "test".to_string(),
        };
        let interactor = BookInteractor;
        let result = interactor.add_book(&mock, &params).await;
        assert_eq!(result.ok(), Some(()));
    }
}
