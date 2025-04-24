use crate::factory::aggregation_worker_factory::{
    AggregationWorkerModule, AggregationWorkerModuleParameters,
};
use crate::factory::server_factory::{ServerModule, ServerModuleParameters};
use crate::factory::user_export_worker_factory::{
    UserExportWorkerModule, UserExportWorkerModuleParameters,
};
use rust_job_server_application::queue::queue::QueueType;
use rust_job_server_config::Config;
use rust_job_server_infrastructure::job::queue::factory::sqs_client_factory::{
    SqsClientFactory, SqsClientFactoryError,
};
use rust_job_server_infrastructure::job::queue::sqs::client::SqsClient;
use rust_job_server_interface::job::server::Server;
use rust_job_server_interface::job::worker::Worker;
use shaku::HasComponent;
use std::sync::Arc;
use thiserror::Error;

pub struct JobContainer {}

impl JobContainer {
    pub async fn build_server(config: Config) -> Result<Arc<dyn Server>, JobContainerError> {
        let factory = SqsClientFactory::new(
            config.queue().base_url().to_string(),
            config.aws().region().to_string(),
            *config.queue().wait_time_seconds(),
        );

        let sqs_client = factory
            .build()
            .await
            .map_err(JobContainerError::BuildQueueClientError)?;

        let sqs_client = Arc::new(sqs_client);

        let parameters = ServerModuleParameters::new(vec![
            Self::build_aggregation_worker(sqs_client.clone()).await?,
            Self::build_user_export_worker(sqs_client.clone()).await?,
        ]);

        let module = ServerModule::factory(parameters);
        Ok(module.resolve())
    }

    async fn build_aggregation_worker(
        sqs_client: Arc<SqsClient>,
    ) -> Result<Arc<dyn Worker>, JobContainerError> {
        let url = sqs_client
            .get_queue_url(&QueueType::Aggregation.queue_name())
            .await
            .map_err(|_| JobContainerError::FetchQueueUrlError)?;

        let parameters = AggregationWorkerModuleParameters::new(url, sqs_client);
        let module = AggregationWorkerModule::factory(parameters);

        Ok(module.resolve())
    }

    async fn build_user_export_worker(
        sqs_client: Arc<SqsClient>,
    ) -> Result<Arc<dyn Worker>, JobContainerError> {
        let url = sqs_client
            .get_queue_url(&QueueType::UserExport.queue_name())
            .await
            .map_err(|_| JobContainerError::FetchQueueUrlError)?;

        let parameters = UserExportWorkerModuleParameters::new(url, sqs_client);
        let module = UserExportWorkerModule::factory(parameters);

        Ok(module.resolve())
    }
}

#[derive(Debug, Error)]
pub enum JobContainerError {
    #[error("BuildQueueClientError {0}")]
    BuildQueueClientError(SqsClientFactoryError),
    #[error("FetchQueueUrlError")]
    FetchQueueUrlError,
    #[error("SendMessageError")]
    SendMessageError,
}
