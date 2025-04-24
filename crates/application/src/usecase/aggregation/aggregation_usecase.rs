use crate::queue::aggregation_queue::AggregationSendResult;
use crate::usecase::aggregation::aggregation_input::AggregationInput;
use crate::usecase::aggregation::aggregation_output::AggregationOutput;
use shaku::Interface;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregationUseCaseError {
    #[error("Aggregation error")]
    AggregationError,

    #[error("Aggregation Send Async error {0}")]
    SendAsyncError(AggregationSendResult),
}

#[async_trait::async_trait]
pub trait AggregationUseCase: Interface + Debug + 'static {
    async fn apply(
        &self,
        input: AggregationInput,
    ) -> Result<AggregationOutput, AggregationUseCaseError>;
}
