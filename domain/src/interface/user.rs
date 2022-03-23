use crate::errors::UserError;
use async_trait::async_trait;
use mockall::*;

#[automock]
#[async_trait(?Send)]
pub trait UserDao {
    async fn create_user(&self, email: &str) -> Result<(), UserError>;
}

#[async_trait(?Send)]
pub trait UserUsecase {
    async fn create_user(&self, user_dao: &dyn UserDao, email: &str) -> Result<(), UserError>;
}
