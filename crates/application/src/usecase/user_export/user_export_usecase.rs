use crate::usecase::user_export::user_export_input::UserExportInput;
use crate::usecase::user_export::user_export_output::UserExportOutput;
use shaku::Interface;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserExportError {
    #[error("UserExport error")]
    UserExportError,
}

pub trait UserExportUseCase: Interface {
    fn apply(&self, input: UserExportInput) -> Result<UserExportOutput, UserExportError>;
}
