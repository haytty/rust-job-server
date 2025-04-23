use derive_more::{Constructor, Display};
use getset::Getters;
use rust_job_server_core::model::user::UserId;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregationQueueError {
    #[error("Send Error {0}")]
    SendError(String),
    #[error("Receive Error {0}")]
    ReceiveError(String),
    #[error("Delete Error {0}")]
    DeleteError(String),
}

#[derive(Debug, Constructor, Getters, Display)]
pub struct AggregationSendResult {}

#[derive(Debug)]
pub enum AggregationReceiveResult {
    Received(AggregationReceiveResultReceived),
    NoMessage,
}

#[derive(Debug, Constructor, Getters)]
pub struct AggregationReceiveResultReceived {
    #[get = "pub"]
    user_id: UserId,
    #[get = "pub"]
    delete_key: String,
}

#[derive(Debug, Constructor, Getters, Display)]
pub struct AggregationDeleteResult {}

#[async_trait::async_trait]
pub trait AggregationQueue: Send + Sync + 'static {
    async fn send_message(
        &self,
        enqueueable: UserId,
    ) -> Result<AggregationSendResult, AggregationQueueError>;
    async fn receive_message(&self) -> Result<AggregationReceiveResult, AggregationQueueError>;
    async fn delete_message(
        &self,
        receipt_handle: String,
    ) -> Result<AggregationDeleteResult, AggregationQueueError>;
}
