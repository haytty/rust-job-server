use anyhow::{anyhow, Result};
use aws_sdk_sqs::config::BehaviorVersion;
use aws_sdk_sqs::config::Builder as SqsConfigBuilder;
use aws_sdk_sqs::Client as AwsSqsClient;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;
use url::Url;

pub async fn build_client(queue_url: Url) -> Result<AwsSqsClient> {
    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let config = SqsConfigBuilder::from(&sdk_config)
        .endpoint_url(queue_url)
        .build();

    let client = AwsSqsClient::from_conf(config);

    Ok(client)
}

pub async fn get_queue_url(client: Arc<AwsSqsClient>, queue_name: String) -> Result<String> {
    let result = client.get_queue_url().queue_name(queue_name).send().await?;
    let queue_url = result.queue_url.ok_or_else(|| anyhow!("Queue not found"))?;
    Ok(queue_url)
}

pub async fn send_message<T>(client: Arc<AwsSqsClient>, queue_url: String, message: T) -> Result<()>
where
    T: Serialize,
{
    let message_string = serde_json::to_string(&message)?;

    client
        .send_message()
        .queue_url(queue_url)
        .message_body(message_string)
        .send()
        .await?;

    Ok(())
}

pub async fn receive_message<T>(
    client: Arc<AwsSqsClient>,
    queue_url: String,
) -> Result<(Option<T>, Option<String>)>
where
    T: DeserializeOwned,
{
    let resp = client
        .receive_message()
        .queue_url(queue_url)
        .max_number_of_messages(1)
        .wait_time_seconds(20)
        .send()
        .await?;

    if let Some(messages) = resp.messages {
        if let Some(message) = messages.first() {
            let receipt_handle = message
                .clone()
                .receipt_handle
                .ok_or_else(|| anyhow!("Receipt handle not found"))?;

            if let Some(body) = &message.body {
                let deserialized = serde_json::from_str::<T>(body)?;

                return Ok((Some(deserialized), Some(receipt_handle)));
            }
        }
    }

    Ok((None, None))
}

pub async fn delete_message(
    client: Arc<AwsSqsClient>,
    queue_url: String,
    receipt_handler: String,
) -> Result<()> {
    client
        .delete_message()
        .queue_url(queue_url)
        .receipt_handle(receipt_handler)
        .send()
        .await?;

    Ok(())
}
