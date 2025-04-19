use crate::queue::aws_sqs_modules::{delete_message, get_queue_url, receive_message, send_message};
use crate::queue::dto::example_dto::ExampleDto;
use anyhow::{anyhow, Result};
use aws_sdk_sqs::Client as AwsSqsClient;
use rust_job_server_core::queue::example_queue::ExampleQueue;
use rust_job_server_core::value_object::example::Example;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct SqsExampleQueue {
    client: Arc<AwsSqsClient>,
}

impl SqsExampleQueue {
    pub fn new(client: Arc<AwsSqsClient>) -> Self {
        Self { client }
    }

    pub const fn queue_name(&self) -> &'static str {
        "example_queue"
    }
}

#[async_trait::async_trait]
impl ExampleQueue for SqsExampleQueue {
    async fn send(&self, example: Example) -> Result<()> {
        let queue_url = get_queue_url(self.client.clone(), self.queue_name().to_string()).await?;
        let dto = ExampleDto::new(example.name().to_owned(), example.message().to_owned());
        let _ = send_message(self.client.clone(), queue_url, dto).await?;

        Ok(())
    }

    async fn receive_message(&self) -> Result<(Option<Example>, Option<String>)> {
        let queue_url = get_queue_url(self.client.clone(), self.queue_name().to_string()).await?;
        let (example_dto_opt, receipt_handler_opt) =
            receive_message::<ExampleDto>(self.client.clone(), queue_url).await?;
        match example_dto_opt {
            Some(dto) => {
                let example = Example::new(dto.name().to_owned(), dto.message().to_owned());
                Ok((Some(example), receipt_handler_opt))
            }
            None => Ok((None, None)),
        }
    }

    async fn delete_message(&self, receipt_handler: String) -> Result<()> {
        let queue_url = get_queue_url(self.client.clone(), self.queue_name().to_string()).await?;
        let result = delete_message(self.client.clone(), queue_url, receipt_handler).await?;

        Ok(())
    }
}
