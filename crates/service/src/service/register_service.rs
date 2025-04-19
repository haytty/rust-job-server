use crate::service::Service;
use anyhow::Result;
use rust_job_server_core::queue::example_queue::ExampleQueue;
use rust_job_server_core::value_object::example::Example;
use shaku::Component;

#[derive(Debug, Component)]
#[shaku(interface = Service)]
pub struct RegisterService<Q>
where
    Q: ExampleQueue,
{
    example_queue: Q,
}

#[async_trait::async_trait]
impl<Q> Service for RegisterService<Q>
where
    Q: ExampleQueue,
{
    async fn run(&self) -> Result<()> {
        let example = Example::new("example".to_string(), "example message".to_string());
        let result = self.example_queue.send(example).await?;

        println!("Message enqueued!");

        Ok(())
    }
}
