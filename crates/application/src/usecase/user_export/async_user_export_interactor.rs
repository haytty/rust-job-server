use crate::queue::user_export_queue::UserExportQueue;
use crate::repository::user_repository::UserRepository;
use crate::usecase::user_export::user_export_input::UserExportInput;
use crate::usecase::user_export::user_export_output::UserExportOutput;
use crate::usecase::user_export::user_export_usecase::{UserExportError, UserExportUseCase};
use derive_more::Constructor;
use getset::Getters;
use tracing::info;

#[derive(Debug, Constructor)]
pub struct AsyncUserExportInteractor<Q>
where
    Q: UserExportQueue,
{
    queue: Q,
}

#[async_trait::async_trait]
impl<Q> UserExportUseCase for AsyncUserExportInteractor<Q>
where
    Q: UserExportQueue,
{
    async fn apply(&self, input: UserExportInput) -> Result<UserExportOutput, UserExportError> {
        let result = self
            .queue
            .send_message(input.user_id().to_owned())
            .await
            .map_err(|e| UserExportError::UserExportError)?;

        info!("Enqueue UserExport!!!");

        Ok(UserExportOutput::new())
    }
}
