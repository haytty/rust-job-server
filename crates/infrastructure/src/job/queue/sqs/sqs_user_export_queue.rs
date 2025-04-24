use crate::job::queue::dto::user_export_dto::UserExportDto;
use crate::job::queue::sqs::client::SqsClient;
use derive_more::Constructor;
use rust_job_server_application::queue::user_export_queue::{
    UserExportDeleteResult, UserExportQueue, UserExportQueueError, UserExportReceiveResult,
    UserExportReceiveResultReceived, UserExportSendResult,
};
use rust_job_server_core::model::user::UserId;
use shaku::Component;
use std::fmt::Debug;
use std::sync::Arc;
use url::Url;

#[derive(Debug, Constructor, Component)]
#[shaku(interface = UserExportQueue)]
pub struct SqsUserExportQueue {
    url: Url,
    client: Arc<SqsClient>,
}

#[async_trait::async_trait]
impl UserExportQueue for SqsUserExportQueue {
    async fn send_message(
        &self,
        user_id: UserId,
    ) -> Result<UserExportSendResult, UserExportQueueError> {
        let dto = UserExportDto::new(user_id.to_string());

        let _ = self
            .client
            .send_message(self.url.clone(), dto)
            .await
            .map_err(|e| UserExportQueueError::SendError(e.to_string()))?;

        Ok(UserExportSendResult::new())
    }

    async fn receive_message(&self) -> Result<UserExportReceiveResult, UserExportQueueError> {
        let opt = self
            .client
            .receive_message::<UserExportDto>(self.url.clone())
            .await
            .map_err(|e| UserExportQueueError::ReceiveError(e.to_string()))?;

        match opt {
            Some((dto, receipt_handle)) => {
                let user_id = UserId::from_string(dto.user_id().to_owned()).map_err(|e| {
                    UserExportQueueError::ReceiveError("Invalid user id".to_string())
                })?;
                Ok(UserExportReceiveResult::Received(
                    UserExportReceiveResultReceived::new(user_id, receipt_handle),
                ))
            }
            None => Ok(UserExportReceiveResult::NoMessage),
        }
    }

    async fn delete_message(
        &self,
        receipt_handle: String,
    ) -> Result<UserExportDeleteResult, UserExportQueueError> {
        let _ = self
            .client
            .delete_message(self.url.clone(), receipt_handle.clone())
            .await
            .map_err(|e| UserExportQueueError::DeleteError(e.to_string()))?;

        Ok(UserExportDeleteResult::new())
    }
}
