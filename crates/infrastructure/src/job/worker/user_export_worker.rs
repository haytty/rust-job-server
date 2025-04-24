use crate::job::worker::{Worker, WorkerError};
use rust_job_server_application::queue::user_export_queue::{
    UserExportQueue, UserExportReceiveResult,
};
use rust_job_server_interface::job::handler::user_export::user_export_handler::{
    UserExportHandleInput, UserExportHandler,
};
use shaku::Component;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Component)]
#[shaku(interface = Worker)]
pub struct UserExportWorker {
    #[shaku(inject)]
    queue: Arc<dyn UserExportQueue>,
    #[shaku(inject)]
    handler: Arc<dyn UserExportHandler>,
}

impl UserExportWorker {
    pub fn new(queue: Arc<dyn UserExportQueue>, handler: Arc<dyn UserExportHandler>) -> Self {
        Self { queue, handler }
    }
}

#[async_trait::async_trait]
impl Worker for UserExportWorker {
    async fn run(&self) -> Result<(), WorkerError> {
        loop {
            let result = self
                .queue
                .receive_message()
                .await
                .map_err(|e| WorkerError::QueueReceiveMessageError(e.to_string()))?;

            match result {
                UserExportReceiveResult::Received(user_export_received_result) => {
                    let handle_input = UserExportHandleInput::new(
                        user_export_received_result.user_id().to_string(),
                    );

                    let handle_output = self
                        .handler
                        .handle(handle_input)
                        .await
                        .map_err(|e| WorkerError::HandleError(e.to_string()))?;

                    self.queue
                        .delete_message(user_export_received_result.delete_key().to_owned())
                        .await
                        .map_err(|e| WorkerError::QueueDeleteMessageError(e.to_string()))?;
                    info!("{}", "メッセージの処理が完了しました");
                }

                UserExportReceiveResult::NoMessage => {
                    info!("{}", "No message");
                }
            }
        }
    }
}
