use rust_job_server_interface::job::server::{Server, ServerError};
use rust_job_server_interface::job::worker::Worker;
use shaku::{Component, Interface};
use std::sync::Arc;

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
