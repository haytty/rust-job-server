use crate::job::queue::dto::aggregation_dto::AggregationDto;
use crate::job::queue::sqs::client::SqsClient;
use derive_more::Constructor;
use rust_job_server_application::queue::aggregation_queue::{
    AggregationDeleteResult, AggregationQueue, AggregationReceiveResult,
    AggregationReceiveResultReceived, AggregationSendResult,
};
use rust_job_server_application::queue::queue::QueueError;
use rust_job_server_core::model::user::UserId;
use shaku::Component;
use std::fmt::Debug;
use std::sync::Arc;
use url::Url;

#[derive(Debug, Constructor, Component)]
#[shaku(interface = AggregationQueue)]
pub struct SqsAggregationQueue {
    url: Url,
    client: Arc<SqsClient>,
}

#[async_trait::async_trait]
impl AggregationQueue for SqsAggregationQueue {
    async fn send_message(&self, user_id: UserId) -> Result<AggregationSendResult, QueueError> {
        let dto = AggregationDto::new(user_id.to_string());

        let _ = self
            .client
            .send_message(self.url.clone(), dto)
            .await
            .map_err(|e| QueueError::SendError(e.to_string()))?;

        Ok(AggregationSendResult::new())
    }

    async fn receive_message(&self) -> Result<AggregationReceiveResult, QueueError> {
        let opt = self
            .client
            .receive_message::<AggregationDto>(self.url.clone())
            .await
            .map_err(|e| QueueError::ReceiveError(e.to_string()))?;

        match opt {
            Some((dto, receipt_handle)) => {
                let user_id = UserId::from_string(dto.user_id().to_owned())
                    .map_err(|e| QueueError::ReceiveError("Invalid user id".to_string()))?;
                Ok(AggregationReceiveResult::Received(
                    AggregationReceiveResultReceived::new(user_id, receipt_handle),
                ))
            }
            None => Ok(AggregationReceiveResult::NoMessage),
        }
    }

    async fn delete_message(
        &self,
        receipt_handle: String,
    ) -> Result<AggregationDeleteResult, QueueError> {
        let _ = self
            .client
            .delete_message(self.url.clone(), receipt_handle.clone())
            .await
            .map_err(|e| QueueError::DeleteError(e.to_string()))?;

        Ok(AggregationDeleteResult::new())
    }
}
