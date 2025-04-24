use rust_job_server_interface::job::server::{Server, ServerError};
use rust_job_server_interface::job::worker::Worker;
use shaku::{Component, Interface};
use std::sync::Arc;
use std::time::Duration;

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
                        // 500系のエラーが出たら2分待機してWorkerの処理を再開する
                        tracing::error!("worker error: {:?}", e);
                        tokio::time::sleep(Duration::from_secs(120)).await;
                    }
                }
            });
        }

        // 待機
        futures::future::pending::<()>().await;
        Ok(())
    }
}
