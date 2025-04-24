use crate::job::worker::{Worker, WorkerError};
use rust_job_server_application::queue::aggregation_queue::{
    AggregationQueue, AggregationReceiveResult,
};
use rust_job_server_interface::job::handler::aggregation::aggregation_handler::{
    AggregationHandleInput, AggregationHandler,
};
use shaku::Component;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Component)]
#[shaku(interface = Worker)]
pub struct AggregationWorker {
    #[shaku(inject)]
    queue: Arc<dyn AggregationQueue>,
    #[shaku(inject)]
    handler: Arc<dyn AggregationHandler>,
}

impl AggregationWorker {
    pub fn new(queue: Arc<dyn AggregationQueue>, handler: Arc<dyn AggregationHandler>) -> Self {
        Self { queue, handler }
    }
}

#[async_trait::async_trait]
impl Worker for AggregationWorker {
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
