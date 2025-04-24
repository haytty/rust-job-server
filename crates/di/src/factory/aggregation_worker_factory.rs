use rust_job_server_application::usecase::aggregation::aggregation_from_file_interactor::AggregationFromFileInteractor;

use rust_job_server_infrastructure::job::queue::sqs::client::SqsClient;
use rust_job_server_infrastructure::job::queue::sqs::sqs_aggregation_queue::{
    SqsAggregationQueue, SqsAggregationQueueParameters,
};
use rust_job_server_infrastructure::job::worker::aggregation_worker::AggregationWorker;
use rust_job_server_infrastructure::repository::file_user_repository::{
    FileUserRepository, FileUserRepositoryParameters,
};
use rust_job_server_interface::job::handler::aggregation::aggregation_handler::AggregationHandlerImpl;
use shaku::module;
use std::sync::Arc;
use url::Url;

// モジュール構築用の `shaku` 定義
module! {
    pub AggregationWorkerModule {
        components = [
            AggregationWorker,
            SqsAggregationQueue,
            AggregationHandlerImpl,
            AggregationFromFileInteractor,
            FileUserRepository
        ],
        providers = []
    }
}

// モジュール構築用のパラメータ
#[derive(Debug)]
pub struct AggregationWorkerModuleParameters {
    pub queue_url: Url,
    pub sqs_client: Arc<SqsClient>,
}

impl AggregationWorkerModuleParameters {
    pub fn new(queue_url: Url, sqs_client: Arc<SqsClient>) -> Self {
        Self {
            queue_url,
            sqs_client,
        }
    }
}

impl AggregationWorkerModule {
    pub fn factory(parameters: AggregationWorkerModuleParameters) -> AggregationWorkerModule {
        let builder = AggregationWorkerModule::builder()
            .with_component_parameters::<SqsAggregationQueue>(SqsAggregationQueueParameters {
                url: parameters.queue_url.clone(),
                client: parameters.sqs_client.clone(),
            })
            .with_component_parameters::<FileUserRepository>(FileUserRepositoryParameters {});

        builder.build()
    }
}
