use crate::job::worker::{Worker, WorkerError};
use shaku::{Component, Interface};
use std::sync::Arc;
use thiserror::Error;

#[async_trait::async_trait]
pub trait Server: Interface {
    async fn run(&self) -> Result<(), ServerError>;
}

#[derive(Component)]
#[shaku(interface = Server)]
pub struct BasicServer {
    workers: Vec<Arc<dyn Worker>>,
}

impl BasicServer {
    pub fn new(workers: Vec<Arc<dyn Worker>>) -> Self {
        Self { workers }
    }
}

#[async_trait::async_trait]
impl Server for BasicServer {
    async fn run(&self) -> Result<(), ServerError> {
        for worker in &self.workers {
            let worker = worker.clone();
            tokio::spawn(async move {
                loop {
                    if let Err(e) = worker.run().await {
                        // エラーをログにだけ出して、止まらず再開
                        tracing::error!("worker error: {:?}", e);
                    }
                }
            });
        }

        // 待機
        futures::future::pending::<()>().await;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("worker failed: {0}")]
    WorkerFailed(#[from] WorkerError),
}
