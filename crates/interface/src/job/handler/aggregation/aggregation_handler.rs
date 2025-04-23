use crate::job::handler::{HandleInput, HandleOutput, Handler, HandlerError};
use derive_more::Constructor;
use getset::Getters;
use rust_job_server_application::usecase::aggregation::aggregation_input::{
    AggregationInput, AggregationInputError,
};
use rust_job_server_application::usecase::aggregation::aggregation_output::AggregationOutput;
use rust_job_server_application::usecase::aggregation::aggregation_usecase::{
    AggregationError, AggregationUseCase,
};

use thiserror::Error;
use tracing::info;

#[derive(Debug, Constructor, Getters)]
pub struct AggregationHandler<U>
where
    U: AggregationUseCase,
{
    use_case: U,
}

impl<U> Handler<AggregationHandleInput, AggregationHandleOutput, AggregationHandlerError>
    for AggregationHandler<U>
where
    U: AggregationUseCase,
{
    fn handle(
        &self,
        handle_input: AggregationHandleInput,
    ) -> Result<AggregationHandleOutput, AggregationHandlerError> {
        let use_case_input = handle_input
            .to_use_case_input()
            .map_err(AggregationHandlerError::AggregationConvertInputError)?;

        let aggregation_output = self
            .use_case
            .apply(use_case_input)
            .map_err(AggregationHandlerError::AggregationUseCaseError)?;

        info!("AggregationHandler: handling aggregation input...");

        let output = AggregationHandleOutput::from_use_case_output(aggregation_output)
            .map_err(|_| AggregationHandlerError::AggregationConvertOutputError)?;

        Ok(output)
    }
}

#[derive(Debug, Error)]
pub enum AggregationHandlerError {
    #[error("AggregationConvertInputError {0}")]
    AggregationConvertInputError(AggregationInputError),
    #[error("AggregationUseCaseError {0}")]
    AggregationUseCaseError(AggregationError),
    #[error("AggregationConvertOutputError")]
    AggregationConvertOutputError,
}

impl HandlerError for AggregationHandlerError {}

#[derive(Debug, Constructor, Getters)]
pub struct AggregationHandleOutput {}

impl AggregationHandleOutput {}

impl HandleOutput<AggregationOutput, AggregationHandlerError> for AggregationHandleOutput {
    fn from_use_case_output(
        use_case_output: AggregationOutput,
    ) -> Result<Self, AggregationHandlerError> {
        Ok(AggregationHandleOutput {})
    }
}

#[derive(Debug, Constructor, Getters)]
pub struct AggregationHandleInput {
    #[get = "pub"]
    user_id: String,
}

impl HandleInput<AggregationInput, AggregationInputError> for AggregationHandleInput {
    fn to_use_case_input(self) -> Result<AggregationInput, AggregationInputError> {
        AggregationInput::from_user_id_string(self.user_id)
    }
}
