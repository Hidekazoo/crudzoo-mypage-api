use async_trait::async_trait;
use domain::errors::UserError;
use domain::interface::{UserDao, DB};

#[derive(Copy, Clone)]
pub struct UserRepository<T: DB> {
    pub db: T,
}

#[async_trait(?Send)]
impl<T: DB> UserDao for UserRepository<T> {
    async fn create_user(&self, email: &str) -> Result<(), UserError> {
        self.db.create_user(email).await?;
        Ok(())
    }
}
