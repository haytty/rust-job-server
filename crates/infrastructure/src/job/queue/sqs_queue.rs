use aws_sdk_sqs::operation::delete_message::DeleteMessageOutput;
use aws_sdk_sqs::operation::send_message::SendMessageOutput;
use aws_sdk_sqs::Client as AwsSqsClient;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;
use thiserror::Error;
use url::Url;

#[derive(Debug)]
pub struct SqsQueue {
    url: Url,
    client: Arc<AwsSqsClient>,
}

impl SqsQueue {
    pub fn new(url: Url, client: Arc<AwsSqsClient>) -> Self {
        Self { url, client }
    }
}

type ReceiptHandle = String;

impl SqsQueue {
    pub async fn send<S: Serialize>(
        &self,
        serializable: S,
    ) -> Result<SendMessageOutput, SqsQueueError> {
        let message_string = serde_json::to_string(&serializable)
            .map_err(|_| SqsQueueError::JsonSerializationError)?;

        let output = self
            .client
            .send_message()
            .queue_url(self.url.as_str())
            .message_body(message_string)
            .send()
            .await
            .map_err(|_| SqsQueueError::SendMessageError)?;

        Ok(output)
    }

    pub async fn receive_message<D: DeserializeOwned>(
        &self,
    ) -> Result<(Option<D>, Option<ReceiptHandle>), SqsQueueError> {
        let resp = self
            .client
            .receive_message()
            .queue_url(self.url.as_str())
            .max_number_of_messages(1)
            .wait_time_seconds(20)
            .send()
            .await
            .map_err(|_| SqsQueueError::ReceiveMessageError)?;

        if let Some(messages) = resp.messages {
            if let Some(message) = messages.first() {
                let receipt_handle = message
                    .clone()
                    .receipt_handle
                    .ok_or(SqsQueueError::ReceiptHandlerMissing)?;

                if let Some(body) = &message.body {
                    let deserialized = serde_json::from_str::<D>(body)
                        .map_err(|_| SqsQueueError::JsonDeserializationError)?;

                    return Ok((Some(deserialized), Some(receipt_handle)));
                }
            }
        }

        Ok((None, None))
    }

    pub async fn delete_message(
        &self,
        receipt_handle: ReceiptHandle,
    ) -> Result<DeleteMessageOutput, SqsQueueError> {
        let output = self
            .client
            .delete_message()
            .queue_url(self.url.as_str())
            .receipt_handle(receipt_handle)
            .send()
            .await
            .map_err(|_| SqsQueueError::DeleteMessageError)?;

        Ok(output)
    }
}

#[derive(Debug, Error)]
pub enum SqsQueueError {
    #[error("Json serialization error")]
    JsonSerializationError,
    #[error("Json deserialization error")]
    JsonDeserializationError,
    #[error("Send message error")]
    SendMessageError,
    #[error("Receive message error")]
    ReceiveMessageError,
    #[error("Delete message error")]
    DeleteMessageError,
    #[error("Receipt handler missing")]
    ReceiptHandlerMissing,
}
