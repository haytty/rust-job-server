use aws_sdk_sqs::error::SdkError;
use aws_sdk_sqs::operation::delete_message::{DeleteMessageError, DeleteMessageOutput};
use aws_sdk_sqs::operation::receive_message::ReceiveMessageError;
use aws_sdk_sqs::operation::send_message::{SendMessageError, SendMessageOutput};
use aws_sdk_sqs::Client as AwsSqsClient;
use derive_more::Constructor;
use rust_job_server_application::queue::queue::{Dequeueable, Enqueueable};
use std::str::FromStr;
use thiserror::Error;
use url::Url;

pub type ReceiptHandle = String;

#[derive(Debug, Constructor)]
pub struct SqsClient {
    client: AwsSqsClient,
    max_number_of_messages: i32,
    wait_time_seconds: i32,
}

impl SqsClient {
    pub async fn send_message<E: Enqueueable>(
        &self,
        url: Url,
        enqueueable: E,
    ) -> Result<SendMessageOutput, SqsClientError> {
        let message_string = serde_json::to_string(&enqueueable)
            .map_err(|e| SqsClientError::JsonSerializationError(e.to_string()))?;

        let output = self
            .client
            .send_message()
            .queue_url(url.as_str())
            .message_body(message_string)
            .send()
            .await
            .map_err(|e| SqsClientError::SendMessageError(e.into_service_error()))?;

        Ok(output)
    }

    pub async fn receive_message<D: Dequeueable>(
        &self,
        url: Url,
    ) -> Result<Option<(D, ReceiptHandle)>, SqsClientError> {
        let resp = self
            .client
            .receive_message()
            .queue_url(url.as_str())
            .max_number_of_messages(self.max_number_of_messages)
            .wait_time_seconds(self.wait_time_seconds)
            .send()
            .await
            .map_err(|e| SqsClientError::ReceiveMessageError(e.into_service_error()))?;

        if let Some(messages) = resp.messages {
            if let Some(message) = messages.first() {
                let receipt_handle = message
                    .clone()
                    .receipt_handle
                    .ok_or(SqsClientError::ReceiptHandlerMissing)?;

                if let Some(body) = &message.body {
                    let deserialized = serde_json::from_str::<D>(body)
                        .map_err(|e| SqsClientError::JsonDeserializationError(e.to_string()))?;

                    return Ok(Some((deserialized, receipt_handle)));
                }
            }
        }

        Ok(None)
    }

    pub async fn delete_message(
        &self,
        url: Url,
        receipt_handle: String,
    ) -> Result<DeleteMessageOutput, SqsClientError> {
        let output = self
            .client
            .delete_message()
            .queue_url(url.as_str())
            .receipt_handle(receipt_handle)
            .send()
            .await
            .map_err(|e| SqsClientError::DeleteMessageError(e.into_service_error()))?;

        Ok(output)
    }

    pub async fn get_queue_url(&self, queue_name: &str) -> Result<Url, SqsClientError> {
        let output = self
            .client
            .get_queue_url()
            .queue_name(queue_name)
            .send()
            .await
            .map_err(|_| SqsClientError::GetQueueUrlError)?;

        let queue_url = output.queue_url.ok_or(SqsClientError::GetQueueUrlError)?;

        let url =
            Url::from_str(queue_url.as_str()).map_err(|_| SqsClientError::ParseQueueUrlError)?;

        Ok(url)
    }
}

#[derive(Debug, Error)]
pub enum SqsClientError {
    // JSON Error
    #[error("Json serialization error {0}")]
    JsonSerializationError(String),
    #[error("Json deserialization error {0}")]
    JsonDeserializationError(String),

    //
    #[error("Send message error {0}")]
    SendMessageError(SendMessageError),
    #[error("Receive message error {0}")]
    ReceiveMessageError(ReceiveMessageError),
    #[error("Delete message error {0}")]
    DeleteMessageError(DeleteMessageError),
    #[error("Receipt handler missing")]
    ReceiptHandlerMissing,
    #[error("GetQueueUrlError")]
    GetQueueUrlError,
    #[error("ParseQueueUrlError")]
    ParseQueueUrlError,
}
