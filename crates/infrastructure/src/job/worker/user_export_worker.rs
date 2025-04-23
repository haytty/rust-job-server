use crate::job::queue::dto::user_export_dto::UserExportDto;
use crate::job::queue::sqs_queue::SqsQueue;
use crate::job::worker::{Worker, WorkerError};
use rust_job_server_interface::job::handler::user_export::user_export_handler::{
    UserExportHandleInput, UserExportHandleOutput, UserExportHandlerError,
};
use rust_job_server_interface::job::handler::Handler;
use tracing::info;

pub struct UserExportWorker<H>
where
    H: Handler<UserExportHandleInput, UserExportHandleOutput, UserExportHandlerError>,
{
    queue: SqsQueue,
    handler: H,
}

impl<H> UserExportWorker<H>
where
    H: Handler<UserExportHandleInput, UserExportHandleOutput, UserExportHandlerError>,
{
    pub fn new(queue: SqsQueue, handler: H) -> Self {
        Self { queue, handler }
    }
}

#[async_trait::async_trait]
impl<H> Worker for UserExportWorker<H>
where
    H: Handler<UserExportHandleInput, UserExportHandleOutput, UserExportHandlerError>,
{
    async fn run(&self) -> Result<(), WorkerError> {
        loop {
            let result = self
                .queue
                .receive_message::<UserExportDto>()
                .await
                .map_err(WorkerError::QueueReceiveMessageError)?;

            match result {
                (Some(message), Some(receipt_handler)) => {
                    info!("{:?}", message);

                    let result = self
                        .handler
                        .handle(message.into())
                        .map_err(|e| WorkerError::HandleError(e.to_string()))?;

                    self.queue
                        .delete_message(receipt_handler)
                        .await
                        .map_err(WorkerError::QueueDeleteMessageError)?;
                    info!("{}", "メッセージの処理が完了しました");
                }
                _ => {
                    info!("{}", "No message");
                }
            }
        }
    }
}
