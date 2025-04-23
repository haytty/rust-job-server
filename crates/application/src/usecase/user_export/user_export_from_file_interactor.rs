use crate::repository::user_repository::UserRepository;
use crate::usecase::user_export::user_export_input::UserExportInput;
use crate::usecase::user_export::user_export_output::UserExportOutput;
use crate::usecase::user_export::user_export_usecase::{UserExportError, UserExportUseCase};
use derive_more::Constructor;
use getset::Getters;
use tracing::info;

#[derive(Debug, Constructor)]
pub struct UserExportFromFileInteractor<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> UserExportUseCase for UserExportFromFileInteractor<R>
where
    R: UserRepository,
{
    fn apply(&self, input: UserExportInput) -> Result<UserExportOutput, UserExportError> {
        info!("UserExport from file!!!");

        Ok(UserExportOutput::new())
    }
}
