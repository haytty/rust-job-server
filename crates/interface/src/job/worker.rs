use shaku::Interface;
use std::fmt::Debug;
use thiserror::Error;

#[async_trait::async_trait]
pub trait Worker: Interface + Send + Sync + Debug {
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
