use crate::job::worker::WorkerError;
use shaku::Interface;
use thiserror::Error;

#[async_trait::async_trait]
pub trait Server: Interface {
    async fn run(&self) -> Result<(), ServerError>;
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("worker failed: {0}")]
    WorkerFailed(#[from] WorkerError),
}
