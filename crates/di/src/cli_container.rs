use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::Client as AwsSqsClient;
use rust_job_server_application::usecase::aggregation::async_aggregation_interactor::AsyncAggregationInteractor;
use rust_job_server_application::usecase::user_export::async_user_export_interactor::AsyncUserExportInteractor;
use rust_job_server_config::Config;
use rust_job_server_infrastructure::job::queue::sqs::client::SqsClient;
use rust_job_server_infrastructure::job::queue::sqs::sqs_aggregation_queue::SqsAggregationQueue;
use rust_job_server_infrastructure::job::queue::sqs::sqs_user_export_queue::SqsUserExportQueue;
use rust_job_server_interface::cli::handler::aggregation::aggregation_handler::AggregationHandler;
use rust_job_server_interface::cli::handler::user_export::user_export_handler::UserExportHandler;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use url::Url;

pub struct CliContainer {}

impl CliContainer {
    pub async fn build_aggregation_handler(
        config: Config,
    ) -> Result<
        AggregationHandler<AsyncAggregationInteractor<SqsAggregationQueue>>,
        CliContainerError,
    > {
        let client = Self::build_sqs_client(&config).await;

        let aggregation_sqs_queue_url =
            Self::fetch_sqs_queue_url(client.clone(), "aggregation_queue").await?;

        let sqs_client = SqsClient::new(client, 1, *config.queue().wait_time_seconds());
        let sqs_aggregation_queue = SqsAggregationQueue::new(aggregation_sqs_queue_url, sqs_client);

        let aggregation_handler =
            AggregationHandler::new(AsyncAggregationInteractor::new(sqs_aggregation_queue));

        Ok(aggregation_handler)
    }

    pub async fn build_user_export_handler(
        config: Config,
    ) -> Result<UserExportHandler<AsyncUserExportInteractor<SqsUserExportQueue>>, CliContainerError>
    {
        let client = Self::build_sqs_client(&config).await;

        let user_export_sqs_queue_url =
            Self::fetch_sqs_queue_url(client.clone(), "user_export_queue").await?;

        let sqs_client = SqsClient::new(client, 1, *config.queue().wait_time_seconds());
        let user_export_queue = SqsUserExportQueue::new(user_export_sqs_queue_url, sqs_client);

        let user_export_handler =
            UserExportHandler::new(AsyncUserExportInteractor::new(user_export_queue));

        Ok(user_export_handler)
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

    async fn fetch_sqs_queue_url(
        client: Arc<AwsSqsClient>,
        queue_name: &str,
    ) -> Result<Url, CliContainerError> {
        let output = client
            .get_queue_url()
            .queue_name(queue_name)
            .send()
            .await
            .map_err(|_| CliContainerError::GetQueueUrlError)?;

        let queue_url = output
            .queue_url
            .ok_or(CliContainerError::GetQueueUrlError)?;

        let url =
            Url::from_str(queue_url.as_str()).map_err(|_| CliContainerError::ParseQueueUrlError)?;

        Ok(url)
    }
}

#[derive(Debug, Error)]
pub enum CliContainerError {
    #[error("GetQueueUrlError")]
    GetQueueUrlError,
    #[error("ParseQueueUrlError")]
    ParseQueueUrlError,
    #[error("SendMessageError")]
    SendMessageError,
}
