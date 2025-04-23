use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::Client as AwsSqsClient;
use rust_job_server_application::usecase::aggregation::aggregation_from_file_interactor::AggregationFromFileInteractor;
use rust_job_server_application::usecase::user_export::user_export_from_file_interactor::UserExportFromFileInteractor;
use rust_job_server_config::Config;
use rust_job_server_infrastructure::job::queue::sqs::client::SqsClient;
use rust_job_server_infrastructure::job::queue::sqs::sqs_aggregation_queue::SqsAggregationQueue;
use rust_job_server_infrastructure::job::queue::sqs::sqs_user_export_queue::SqsUserExportQueue;
use rust_job_server_infrastructure::job::server::{
    BasicServer, Server, ServerBuilder, ServerBuilderError,
};
use rust_job_server_infrastructure::job::worker::aggregation_worker::AggregationWorker;
use rust_job_server_infrastructure::job::worker::user_export_worker::UserExportWorker;
use rust_job_server_infrastructure::job::worker::Worker;
use rust_job_server_infrastructure::repository::file_user_repository::FileUserRepository;
use rust_job_server_interface::job::handler::aggregation::aggregation_handler::AggregationHandler;
use rust_job_server_interface::job::handler::user_export::user_export_handler::UserExportHandler;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use url::Url;

pub struct JobContainer {}

impl JobContainer {
    pub async fn build_server(config: Config) -> Result<Arc<dyn Server>, JobContainerError> {
        let client = Self::build_sqs_client(&config).await;

        let mut server_builder = ServerBuilder::new();
        let server_builder = server_builder
            .add_worker(Self::build_aggregation_worker(&config, client.clone()).await?)
            .add_worker(Self::build_user_export_worker(&config, client.clone()).await?);

        server_builder
            .build()
            .map_err(JobContainerError::ServerBuilderError)
    }

    async fn build_sqs_client(config: &Config) -> Arc<AwsSqsClient> {
        let url = Url::from_str(config.queue().base_url()).expect("invalid url");
        let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");

        let config = aws_config::from_env().region(region_provider).load().await;

        let client = AwsSqsClient::from_conf(
            aws_sdk_sqs::config::Builder::from(&config)
                .endpoint_url(url.as_str())
                .build(),
        );

        Arc::new(client)
    }

    async fn build_aggregation_worker(
        config: &Config,
        client: Arc<AwsSqsClient>,
    ) -> Result<Arc<dyn Worker>, JobContainerError> {
        let url = Self::fetch_sqs_queue_url(client.clone(), "aggregation_queue").await?;
        let sqs_client = SqsClient::new(client, 1, *config.queue().wait_time_seconds());
        let aggregation_queue = SqsAggregationQueue::new(url, sqs_client);
        let worker = AggregationWorker::new(
            aggregation_queue,
            AggregationHandler::new(AggregationFromFileInteractor::new(FileUserRepository::new())),
        );

        Ok(Arc::new(worker))
    }

    async fn build_user_export_worker(
        config: &Config,
        client: Arc<AwsSqsClient>,
    ) -> Result<Arc<dyn Worker>, JobContainerError> {
        let url = Self::fetch_sqs_queue_url(client.clone(), "user_export_queue").await?;
        let sqs_client = SqsClient::new(client, 1, *config.queue().wait_time_seconds());
        let user_export_queue = SqsUserExportQueue::new(url, sqs_client);
        let worker = UserExportWorker::new(
            user_export_queue,
            UserExportHandler::new(UserExportFromFileInteractor::new(FileUserRepository::new())),
        );

        Ok(Arc::new(worker))
    }

    async fn fetch_sqs_queue_url(
        client: Arc<AwsSqsClient>,
        queue_name: &str,
    ) -> Result<Url, JobContainerError> {
        let output = client
            .get_queue_url()
            .queue_name(queue_name)
            .send()
            .await
            .map_err(|_| JobContainerError::GetQueueUrlError)?;

        let queue_url = output
            .queue_url
            .ok_or(JobContainerError::GetQueueUrlError)?;

        let url =
            Url::from_str(queue_url.as_str()).map_err(|_| JobContainerError::ParseQueueUrlError)?;

        Ok(url)
    }
}

#[derive(Debug, Error)]
pub enum JobContainerError {
    #[error("ServerBuilderError")]
    ServerBuilderError(#[from] ServerBuilderError),
    #[error("GetQueueUrlError")]
    GetQueueUrlError,
    #[error("ParseQueueUrlError")]
    ParseQueueUrlError,
}
