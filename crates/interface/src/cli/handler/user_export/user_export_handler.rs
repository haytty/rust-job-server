use crate::cli::handler::{HandleInput, HandleOutput, Handler, HandlerError};
use derive_more::Constructor;
use getset::Getters;
use rust_job_server_application::usecase::user_export::user_export_input::{
    UserExportInput, UserExportInputError,
};
use rust_job_server_application::usecase::user_export::user_export_output::UserExportOutput;
use rust_job_server_application::usecase::user_export::user_export_usecase::{
    UserExportError, UserExportUseCase,
};

use thiserror::Error;
use tracing::info;

#[derive(Debug, Constructor, Getters)]
pub struct UserExportHandler<U>
where
    U: UserExportUseCase,
{
    use_case: U,
}

#[async_trait::async_trait]
impl<U> Handler<UserExportHandleInput, UserExportHandleOutput, UserExportHandlerError>
    for UserExportHandler<U>
where
    U: UserExportUseCase,
{
    async fn handle(
        &self,
        handle_input: UserExportHandleInput,
    ) -> Result<UserExportHandleOutput, UserExportHandlerError> {
        let use_case_input = handle_input
            .to_use_case_input()
            .map_err(UserExportHandlerError::UserExportConvertInputError)?;

        let user_export_output = self
            .use_case
            .apply(use_case_input)
            .await
            .map_err(UserExportHandlerError::UserExportUseCaseError)?;

        info!("UserExportHandler: handling user_export input...");

        let output = UserExportHandleOutput::from_use_case_output(user_export_output)
            .map_err(|_| UserExportHandlerError::UserExportConvertOutputError)?;

        Ok(output)
    }
}

#[derive(Debug, Error)]
pub enum UserExportHandlerError {
    #[error("UserExportConvertInputError {0}")]
    UserExportConvertInputError(UserExportInputError),
    #[error("UserExportUseCaseError {0}")]
    UserExportUseCaseError(UserExportError),
    #[error("UserExportConvertOutputError")]
    UserExportConvertOutputError,
}

impl HandlerError for UserExportHandlerError {}

#[derive(Debug, Constructor, Getters)]
pub struct UserExportHandleOutput {}

impl UserExportHandleOutput {}

impl HandleOutput<UserExportOutput, UserExportHandlerError> for UserExportHandleOutput {
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

impl HandleInput<UserExportInput, UserExportInputError> for UserExportHandleInput {
    fn to_use_case_input(self) -> Result<UserExportInput, UserExportInputError> {
        UserExportInput::from_user_id_string(self.user_id)
    }
}
