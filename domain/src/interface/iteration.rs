use mockall::automock;

use crate::entity::iteration::Iteration;

#[automock]
#[async_trait::async_trait]
pub trait IterationRepository {
    async fn store(&self, iteration: &Iteration) -> anyhow::Result<()>;
}
