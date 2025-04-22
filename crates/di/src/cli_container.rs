use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::Client as AwsSqsClient;
use rust_job_server_config::Config;
use rust_job_server_infrastructure::job::queue::dto::aggregation_dto::AggregationDto;
use rust_job_server_infrastructure::job::queue::sqs_queue::SqsQueue;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

pub struct CliContainer {}

impl CliContainer {
    pub async fn build_xxx_handler(config: Config) -> Result<(), CliContainerError> {
        let client = Self::build_sqs_client(config).await;

        let sqs_queue_url = Self::fetch_sqs_queue_url(client.clone(), "aggregation_queue").await?;

        let sqs_queue = SqsQueue::new(sqs_queue_url, client);

        let a = Uuid::new_v4();
        let dto = AggregationDto::new(a.to_string());

        let a = sqs_queue
            .send(dto)
            .await
            .map_err(|_| CliContainerError::SendMessageError)?;

        println!("{:?}", a);
        Ok(())
    }

    async fn build_sqs_client(config: Config) -> Arc<AwsSqsClient> {
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
