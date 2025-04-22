use crate::usecase::aggregation::aggregation_input::AggregationInput;
use crate::usecase::aggregation::aggregation_output::AggregationOutput;
use shaku::Interface;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregationError {
    #[error("Aggregation error")]
    AggregationError,
}

pub trait AggregationUseCase: Interface {
    fn apply(&self, input: AggregationInput) -> Result<AggregationOutput, AggregationError>;
}
