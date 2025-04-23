use crate::queue::aggregation_queue::AggregationQueue;
use crate::repository::user_repository::UserRepository;
use crate::usecase::aggregation::aggregation_input::AggregationInput;
use crate::usecase::aggregation::aggregation_output::AggregationOutput;
use crate::usecase::aggregation::aggregation_usecase::{AggregationError, AggregationUseCase};
use derive_more::Constructor;
use getset::Getters;
use tracing::info;

#[derive(Debug, Constructor)]
pub struct AsyncAggregationInteractor<Q>
where
    Q: AggregationQueue,
{
    queue: Q,
}

#[async_trait::async_trait]
impl<Q> AggregationUseCase for AsyncAggregationInteractor<Q>
where
    Q: AggregationQueue,
{
    async fn apply(&self, input: AggregationInput) -> Result<AggregationOutput, AggregationError> {
        let result = self
            .queue
            .send_message(input.user_id().to_owned())
            .await
            .map_err(|e| AggregationError::AggregationError)?;

        info!("Enqueue Aggregation!!!");

        Ok(AggregationOutput::new())
    }
}
