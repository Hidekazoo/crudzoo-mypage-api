use crate::{IterationDriver, IterationStoreParam};
use domain::{entity::iteration::Iteration, interface::iteration::IterationRepository};

#[derive(Debug)]
pub struct IterationService {
    pub iteration_driver: IterationDriver,
}

#[async_trait::async_trait]
impl IterationRepository for IterationService {
    async fn store(&self, iteration: &Iteration) -> Result<(), anyhow::Error> {
        let params = IterationStoreParam {
            start_date: iteration.start_date,
            end_date: iteration.end_date,
            hours: iteration.hours,
        };
        self.iteration_driver.store(params).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Local, Utc};

    use super::*;

    #[tokio::test]
    async fn store_test() {
        let mut iteration_driver = IterationDriver::faux();
        let start_date = Utc::now();
        let end_date = Utc::now();
        let params = IterationStoreParam {
            start_date,
            end_date,
            hours: 24,
        };
        faux::when!(iteration_driver.store(params)).then(|_| Ok(()));

        let iteration_service = IterationService { iteration_driver };
        let iteration = Iteration::new(start_date, end_date, 24);
        let actual = iteration_service.store(&iteration).await.unwrap();
        assert_eq!(actual, ())
    }
}
