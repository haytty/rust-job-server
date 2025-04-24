use crate::repository::user_repository::UserRepository;
use crate::usecase::user_export::user_export_input::UserExportInput;
use crate::usecase::user_export::user_export_output::UserExportOutput;
use crate::usecase::user_export::user_export_usecase::{UserExportUseCase, UserExportUseCaseError};
use derive_more::Constructor;
use shaku::Component;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Constructor, Component)]
#[shaku(interface = UserExportUseCase)]
pub struct UserExportFromFileInteractor {
    #[shaku(inject)]
    repository: Arc<dyn UserRepository>,
}

#[async_trait::async_trait]
impl UserExportUseCase for UserExportFromFileInteractor {
    async fn apply(
        &self,
        input: UserExportInput,
    ) -> Result<UserExportOutput, UserExportUseCaseError> {
        info!("UserExport from file!!!");

        Ok(UserExportOutput::new())
    }
}
