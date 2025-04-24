use derive_more::Constructor;
use getset::Getters;
use rust_job_server_application::usecase::user_export::user_export_input::{
    UserExportInput, UserExportInputError,
};
use rust_job_server_application::usecase::user_export::user_export_output::UserExportOutput;
use rust_job_server_application::usecase::user_export::user_export_usecase::{
    UserExportUseCase, UserExportUseCaseError,
};
use shaku::{Component, Interface};
use std::sync::Arc;
use thiserror::Error;
use tracing::info;

#[async_trait::async_trait]
pub trait UserExportHandler: Interface + Send + Sync {
    async fn handle(
        &self,
        handle_input: UserExportHandleInput,
    ) -> Result<UserExportHandleOutput, UserExportHandlerError>;
}

#[derive(Debug, Constructor, Getters, Component)]
#[shaku(interface = UserExportHandler)]
pub struct UserExportHandlerImpl {
    #[shaku(inject)]
    use_case: Arc<dyn UserExportUseCase>,
}

#[async_trait::async_trait]
impl UserExportHandler for UserExportHandlerImpl {
    async fn handle(
        &self,
        handle_input: UserExportHandleInput,
    ) -> Result<UserExportHandleOutput, UserExportHandlerError> {
        let use_case_input = handle_input
            .to_use_case_input()
            .map_err(UserExportHandlerError::UserExportConvertInputError)?;

        let aggregation_output = self
            .use_case
            .apply(use_case_input)
            .await
            .map_err(UserExportHandlerError::UserExportUseCaseError)?;

        info!("UserExportHandler: handling aggregation input...");

        let output = UserExportHandleOutput::from_use_case_output(aggregation_output)
            .map_err(|_| UserExportHandlerError::UserExportConvertOutputError)?;

        Ok(output)
    }
}

#[derive(Debug, Error)]
pub enum UserExportHandlerError {
    #[error("UserExportConvertInputError {0}")]
    UserExportConvertInputError(UserExportInputError),
    #[error("UserExportUseCaseError {0}")]
    UserExportUseCaseError(UserExportUseCaseError),
    #[error("UserExportConvertOutputError")]
    UserExportConvertOutputError,
}

#[derive(Debug, Constructor, Getters)]
pub struct UserExportHandleOutput {}

impl UserExportHandleOutput {
    fn from_use_case_output(
        use_case_output: UserExportOutput,
    ) -> Result<Self, UserExportHandlerError> {
        Ok(UserExportHandleOutput {})
    }
}

#[derive(Debug, Constructor, Getters)]
pub struct UserExportHandleInput {
    #[get = "pub"]
    user_id: String,
}

impl UserExportHandleInput {
    fn to_use_case_input(self) -> Result<UserExportInput, UserExportInputError> {
        UserExportInput::from_user_id_string(self.user_id)
    }
}
