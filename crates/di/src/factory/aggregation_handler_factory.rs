use rust_job_server_application::usecase::aggregation::async_aggregation_interactor::AsyncAggregationInteractor;
use rust_job_server_infrastructure::job::queue::sqs::client::SqsClient;
use rust_job_server_infrastructure::job::queue::sqs::sqs_aggregation_queue::{
    SqsAggregationQueue, SqsAggregationQueueParameters,
};
use rust_job_server_interface::cli::handler::aggregation::aggregation_handler::AggregationHandlerImpl;
use shaku::module;
use std::sync::Arc;
use url::Url;

module! {
    pub AggregationHandlerModule {
        components = [
            SqsAggregationQueue,
            AggregationHandlerImpl,
            AsyncAggregationInteractor,
        ],
        providers = []
    }
}

#[derive(Debug)]
pub struct AggregationHandlerModuleParameters {
    queue_url: Url,
    sqs_client: Arc<SqsClient>,
}

impl AggregationHandlerModuleParameters {
    pub fn new(queue_url: Url, sqs_client: Arc<SqsClient>) -> Self {
        Self {
            queue_url,
            sqs_client,
        }
    }
}
impl AggregationHandlerModule {
    pub fn factory(parameters: AggregationHandlerModuleParameters) -> AggregationHandlerModule {
        let builder = AggregationHandlerModule::builder()
            .with_component_parameters::<SqsAggregationQueue>(SqsAggregationQueueParameters {
                url: parameters.queue_url.clone(),
                client: parameters.sqs_client.clone(),
            });
        let module = builder.build();
        module
    }
}
