use std::sync::Arc;
use crate::queue::user_export_queue::UserExportQueue;
use crate::usecase::user_export::user_export_input::UserExportInput;
use crate::usecase::user_export::user_export_output::UserExportOutput;
use crate::usecase::user_export::user_export_usecase::{UserExportUseCase, UserExportUseCaseError};
use derive_more::Constructor;
use shaku::Component;
use tracing::info;

#[derive(Debug, Constructor, Component)]
#[shaku(interface = UserExportUseCase)]
pub struct AsyncUserExportInteractor
{
    #[shaku(inject)]
    queue: Arc<dyn UserExportQueue>,
}

#[async_trait::async_trait]
impl UserExportUseCase for AsyncUserExportInteractor
{
    async fn apply(
        &self,
        input: UserExportInput,
    ) -> Result<UserExportOutput, UserExportUseCaseError> {
        let result = self
            .queue
            .send_message(input.user_id().to_owned())
            .await
            .map_err(|e| UserExportUseCaseError::UserExportError)?;

        info!("Enqueue UserExport!!!");

        Ok(UserExportOutput::new())
    }
}
