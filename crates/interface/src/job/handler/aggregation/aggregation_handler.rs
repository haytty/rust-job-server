use derive_more::Constructor;
use getset::Getters;
use rust_job_server_application::usecase::aggregation::aggregation_input::{
    AggregationInput, AggregationInputError,
};
use rust_job_server_application::usecase::aggregation::aggregation_output::AggregationOutput;
use rust_job_server_application::usecase::aggregation::aggregation_usecase::{
    AggregationUseCase, AggregationUseCaseError,
};
use shaku::{Component, Interface};
use std::fmt::Debug;
use std::sync::Arc;
use thiserror::Error;
use tracing::info;

#[async_trait::async_trait]
pub trait AggregationHandler: Interface + Send + Sync + Debug {
    async fn handle(
        &self,
        handle_input: AggregationHandleInput,
    ) -> Result<AggregationHandleOutput, AggregationHandlerError>;
}

#[derive(Debug, Constructor, Getters, Component)]
#[shaku(interface = AggregationHandler)]
pub struct AggregationHandlerImpl {
    #[shaku(inject)]
    use_case: Arc<dyn AggregationUseCase>,
}

#[async_trait::async_trait]
impl AggregationHandler for AggregationHandlerImpl {
    async fn handle(
        &self,
        handle_input: AggregationHandleInput,
    ) -> Result<AggregationHandleOutput, AggregationHandlerError> {
        let use_case_input = handle_input
            .to_use_case_input()
            .map_err(AggregationHandlerError::AggregationConvertInputError)?;

        let aggregation_output = self
            .use_case
            .apply(use_case_input)
            .await
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
    AggregationUseCaseError(AggregationUseCaseError),
    #[error("AggregationConvertOutputError")]
    AggregationConvertOutputError,
}

#[derive(Debug, Constructor, Getters)]
pub struct AggregationHandleOutput {}

impl AggregationHandleOutput {
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

impl AggregationHandleInput {
    fn to_use_case_input(self) -> Result<AggregationInput, AggregationInputError> {
        AggregationInput::from_user_id_string(self.user_id)
    }
}
