use crate::job::queue::sqs::client::SqsClient;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::Client as AwsSqsClient;
use std::str::FromStr;
use derive_more::Constructor;
use thiserror::Error;
use url::Url;

#[derive(Debug, Constructor)]
pub struct SqsClientFactory {
    base_url: String,
    region: String,
    wait_time_seconds: i32,
}

impl SqsClientFactory {
    pub async fn build(&self) -> Result<SqsClient, SqsClientFactoryError> {
        let client = Self::build_sqs_client(&self.base_url, &self.region).await?;
        let sqs_client = SqsClient::new(client, 1, self.wait_time_seconds);

        Ok(sqs_client)
    }

    async fn build_sqs_client(
        base_url_str: &str,
        region: &str,
    ) -> Result<AwsSqsClient, SqsClientFactoryError> {
        let url = Url::from_str(base_url_str).expect("invalid url");
        let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");

        let config = aws_config::from_env().region(region_provider).load().await;

        let client = AwsSqsClient::from_conf(
            aws_sdk_sqs::config::Builder::from(&config)
                .endpoint_url(url.as_str())
                .build(),
        );

        Ok(client)
    }
}

#[derive(Debug, Error)]
pub enum SqsClientFactoryError {

}
