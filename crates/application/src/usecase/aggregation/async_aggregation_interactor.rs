use crate::queue::aggregation_queue::AggregationQueue;
use crate::usecase::aggregation::aggregation_input::AggregationInput;
use crate::usecase::aggregation::aggregation_output::AggregationOutput;
use crate::usecase::aggregation::aggregation_usecase::{
    AggregationUseCase, AggregationUseCaseError,
};
use derive_more::Constructor;
use shaku::Component;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Constructor, Component)]
#[shaku(interface = AggregationUseCase)]
pub struct AsyncAggregationInteractor {
    #[shaku(inject)]
    queue: Arc<dyn AggregationQueue>,
}

#[async_trait::async_trait]
impl AggregationUseCase for AsyncAggregationInteractor {
    async fn apply(
        &self,
        input: AggregationInput,
    ) -> Result<AggregationOutput, AggregationUseCaseError> {
        let result = self
            .queue
            .send_message(input.user_id().to_owned())
            .await
            .map_err(|e| AggregationUseCaseError::AggregationError)?;

        info!("Enqueue Aggregation!!!");

        Ok(AggregationOutput::new())
    }
}
