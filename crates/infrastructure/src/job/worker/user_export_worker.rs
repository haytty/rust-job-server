use crate::job::queue::dto::user_export_dto::UserExportDto;
use crate::job::worker::{Worker, WorkerError};
use rust_job_server_application::queue::user_export_queue::{
    UserExportQueue, UserExportReceiveResult,
};
use rust_job_server_interface::job::handler::user_export::user_export_handler::{
    UserExportHandleInput, UserExportHandleOutput, UserExportHandlerError,
};
use rust_job_server_interface::job::handler::Handler;
use tracing::info;

pub struct UserExportWorker<Q, H>
where
    Q: UserExportQueue,
    H: Handler<UserExportHandleInput, UserExportHandleOutput, UserExportHandlerError>,
{
    queue: Q,
    handler: H,
}

impl<Q, H> UserExportWorker<Q, H>
where
    Q: UserExportQueue,
    H: Handler<UserExportHandleInput, UserExportHandleOutput, UserExportHandlerError>,
{
    pub fn new(queue: Q, handler: H) -> Self {
        Self { queue, handler }
    }
}

#[async_trait::async_trait]
impl<Q, H> Worker for UserExportWorker<Q, H>
where
    Q: UserExportQueue,
    H: Handler<UserExportHandleInput, UserExportHandleOutput, UserExportHandlerError>,
{
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
