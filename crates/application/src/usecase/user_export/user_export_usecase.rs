use crate::usecase::user_export::user_export_input::UserExportInput;
use crate::usecase::user_export::user_export_output::UserExportOutput;
use shaku::Interface;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserExportUseCaseError {
    #[error("UserExport error")]
    UserExportError,
}

#[async_trait::async_trait]
pub trait UserExportUseCase: Interface + Debug {
    async fn apply(
        &self,
        input: UserExportInput,
    ) -> Result<UserExportOutput, UserExportUseCaseError>;
}
