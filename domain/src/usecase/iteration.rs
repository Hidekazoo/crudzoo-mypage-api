use crate::{entity::iteration::Iteration, interface::iteration::IterationRepository};

pub async fn add_iteration(
    iteration_service: impl IterationRepository,
    iteration: Iteration,
) -> anyhow::Result<()> {
    iteration_service.store(&iteration).await
}

#[cfg(test)]
mod tests {

    use crate::interface::iteration::MockIterationRepository;

    use super::*;

    #[tokio::test]
    async fn add_iteration_test() {
        let iteration = Iteration::faux();
        let mut iteration_service = MockIterationRepository::new();

        iteration_service
            .expect_store()
            .times(1)
            .return_once(|_| Ok(()));

        let actual = add_iteration(iteration_service, iteration).await.unwrap();
        assert_eq!(actual, ())
    }
}
