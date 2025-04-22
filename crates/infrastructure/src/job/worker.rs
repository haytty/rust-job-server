pub mod aggregation_worker;

use crate::job::queue::sqs_queue::SqsQueueError;
use crate::job::queue::Dequeueable;
use rust_job_server_interface::job::handler::Handler;
use serde::de::DeserializeOwned;
use thiserror::Error;

#[async_trait::async_trait]
pub trait Worker: Send + Sync {
    async fn run(&self) -> Result<(), WorkerError>;
}

#[derive(Debug, Error)]
pub enum WorkerError {
    #[error("QueueReceiveMessageError {0}")]
    QueueReceiveMessageError(SqsQueueError),
    #[error("QueueDeleteMessageError {0}")]
    QueueDeleteMessageError(SqsQueueError),
    #[error("HandleError {0}")]
    HandleError(String),
}
