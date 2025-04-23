pub mod aggregation_worker;
pub mod user_export_worker;

use thiserror::Error;

#[async_trait::async_trait]
pub trait Worker: Send + Sync {
    async fn run(&self) -> Result<(), WorkerError>;
}

#[derive(Debug, Error)]
pub enum WorkerError {
    #[error("QueueReceiveMessageError {0}")]
    QueueReceiveMessageError(String),
    #[error("QueueDeleteMessageError {0}")]
    QueueDeleteMessageError(String),
    #[error("HandleError {0}")]
    HandleError(String),
}
