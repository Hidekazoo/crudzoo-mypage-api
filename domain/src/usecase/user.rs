use crate::errors::UserError;
use crate::interface::{UserDao, UserUsecase};
use async_trait::async_trait;

pub struct UserInteractor;

#[async_trait(?Send)]
impl UserUsecase for UserInteractor {
    async fn create_user(&self, user_dao: &dyn UserDao, email: &str) -> Result<(), UserError> {
        match user_dao.create_user(email).await {
            Ok(_) => Ok(()),
            _ => Err(UserError::UnexpectedError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::MockUserDao;

    #[tokio::test]
    async fn test_create_user_success() {
        let mut mock = MockUserDao::new();
        mock.expect_create_user().return_const(Ok(()));

        let interactor = UserInteractor;
        let result = interactor.create_user(&mock, "test@example.com").await;
        assert_eq!(result.ok(), Some(()));
    }

    #[tokio::test]
    async fn test_create_user_fail() {
        let mut mock = MockUserDao::new();
        mock.expect_create_user()
            .return_const(Err(UserError::UnexpectedError));
        let interactor = UserInteractor;
        let result = interactor.create_user(&mock, "test@example.com").await;
        assert_eq!(result.err(), Some(UserError::UnexpectedError));
    }
}
