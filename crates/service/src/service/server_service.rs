use crate::service::Service;
use anyhow::Result;
use rust_job_server_core::queue::example_queue::ExampleQueue;
use rust_job_server_core::value_object::example::Example;
use shaku::Component;
use tracing::info;

#[derive(Debug, Component)]
#[shaku(interface = Service)]
pub struct ServerService<Q>
where
    Q: ExampleQueue,
{
    example_queue: Q,
}

#[async_trait::async_trait]
impl<Q> Service for ServerService<Q>
where
    Q: ExampleQueue,
{
    async fn run(&self) -> Result<()> {
        loop {
            match self.example_queue.receive_message().await? {
                (Some(message), Some(receipt_handler)) => {
                    info!("{:?}", message);
                    self.example_queue.delete_message(receipt_handler).await?;
                    info!("{}", "メッセージの処理が完了しました");
                }
                _ => {
                    info!("{}", "No message");
                }
            }
        }
    }
}
