use crate::job::queue::dto::aggregation_dto::AggregationDto;
use crate::job::worker::{Worker, WorkerError};
use rust_job_server_application::queue::aggregation_queue::{
    AggregationQueue, AggregationReceiveResult,
};
use rust_job_server_interface::job::handler::aggregation::aggregation_handler::{
    AggregationHandleInput, AggregationHandleOutput, AggregationHandlerError,
};
use rust_job_server_interface::job::handler::Handler;
use tracing::info;

pub struct AggregationWorker<Q, H>
where
    Q: AggregationQueue,
    H: Handler<AggregationHandleInput, AggregationHandleOutput, AggregationHandlerError>,
{
    queue: Q,
    handler: H,
}

impl<Q, H> AggregationWorker<Q, H>
where
    Q: AggregationQueue,
    H: Handler<AggregationHandleInput, AggregationHandleOutput, AggregationHandlerError>,
{
    pub fn new(queue: Q, handler: H) -> Self {
        Self { queue, handler }
    }
}

#[async_trait::async_trait]
impl<Q, H> Worker for AggregationWorker<Q, H>
where
    Q: AggregationQueue,
    H: Handler<AggregationHandleInput, AggregationHandleOutput, AggregationHandlerError>,
{
    async fn run(&self) -> Result<(), WorkerError> {
        loop {
            let result = self
                .queue
                .receive_message()
                .await
                .map_err(|e| WorkerError::QueueReceiveMessageError(e.to_string()))?;

            match result {
                AggregationReceiveResult::Received(aggregation_received_result) => {
                    let handle_input = AggregationHandleInput::new(
                        aggregation_received_result.user_id().to_string(),
                    );

                    let handle_output = self
                        .handler
                        .handle(handle_input)
                        .await
                        .map_err(|e| WorkerError::HandleError(e.to_string()))?;

                    self.queue
                        .delete_message(aggregation_received_result.delete_key().to_owned())
                        .await
                        .map_err(|e| WorkerError::QueueDeleteMessageError(e.to_string()))?;
                    info!("{}", "メッセージの処理が完了しました");
                }

                AggregationReceiveResult::NoMessage => {
                    info!("{}", "No message");
                }
            }
        }
    }
}
