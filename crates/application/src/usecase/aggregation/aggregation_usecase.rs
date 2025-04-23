use crate::queue::aggregation_queue::AggregationSendResult;
use crate::usecase::aggregation::aggregation_input::AggregationInput;
use crate::usecase::aggregation::aggregation_output::AggregationOutput;
use shaku::Interface;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregationError {
    #[error("Aggregation error")]
    AggregationError,

    #[error("Aggregation Send Async error {0}")]
    SendAsyncError(AggregationSendResult),
}

#[async_trait::async_trait]
pub trait AggregationUseCase: Interface {
    async fn apply(&self, input: AggregationInput) -> Result<AggregationOutput, AggregationError>;
}
