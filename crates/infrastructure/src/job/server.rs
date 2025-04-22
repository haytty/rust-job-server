use crate::job::worker::{Worker, WorkerError};
use shaku::{module, Component, HasComponent, Interface};
use std::sync::Arc;
use thiserror::Error;

module! {
    pub ServerModule {
        components = [BasicServer],
        providers = []
    }
}

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

pub struct ServerBuilder {
    workers: Vec<Arc<dyn Worker>>,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self { workers: vec![] }
    }
}

impl ServerBuilder {
    pub fn add_worker(mut self, worker: Arc<dyn Worker>) -> Self {
        self.workers.push(worker);
        self
    }

    pub fn build(self) -> Result<Arc<dyn Server>, ServerBuilderError> {
        let mut server_module_builder = ServerModule::builder();

        let server_module_builder =
            server_module_builder.with_component_parameters::<BasicServer>(BasicServerParameters {
                workers: self.workers,
            });

        let module = server_module_builder.build();
        let server: Arc<dyn Server> = module.resolve();

        Ok(server)
    }
}

#[derive(Debug, Error)]
pub enum ServerBuilderError {}
