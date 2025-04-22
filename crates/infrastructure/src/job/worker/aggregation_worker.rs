use crate::job::queue::dto::aggregation_dto::AggregationDto;
use crate::job::queue::sqs_queue::SqsQueue;
use crate::job::worker::{Worker, WorkerError};
use rust_job_server_interface::job::handler::aggregation::aggregation_handler::{
    AggregationHandleInput, AggregationHandleOutput, AggregationHandlerError,
};
use rust_job_server_interface::job::handler::Handler;
use tracing::info;

pub struct AggregationWorker<H>
where
    H: Handler<AggregationHandleInput, AggregationHandleOutput, AggregationHandlerError>,
{
    queue: SqsQueue,
    handler: H,
}

impl<H> AggregationWorker<H>
where
    H: Handler<AggregationHandleInput, AggregationHandleOutput, AggregationHandlerError>,
{
    pub fn new(queue: SqsQueue, handler: H) -> Self {
        Self { queue, handler }
    }
}

#[async_trait::async_trait]
impl<H> Worker for AggregationWorker<H>
where
    H: Handler<AggregationHandleInput, AggregationHandleOutput, AggregationHandlerError>,
{
    async fn run(&self) -> Result<(), WorkerError> {
        loop {
            let result = self
                .queue
                .receive_message::<AggregationDto>()
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
