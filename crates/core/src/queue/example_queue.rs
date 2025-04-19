use crate::value_object::example::Example;
use anyhow::Result;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait ExampleQueue: Send + Sync + Debug + 'static {
    async fn send(&self, example: Example) -> Result<()>;
    async fn receive_message(&self) -> Result<(Option<Example>, Option<String>)>;
    async fn delete_message(&self, receipt_handler: String) -> Result<()>;
}
