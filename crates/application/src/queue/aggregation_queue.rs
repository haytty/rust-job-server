use crate::queue::queue::QueueError;
use derive_more::{Constructor, Display};
use getset::Getters;
use rust_job_server_core::model::user::UserId;
use shaku::Interface;
use std::fmt::Debug;

#[derive(Debug, Constructor, Getters, Display)]
pub struct AggregationSendResult {}

#[derive(Debug)]
pub enum AggregationReceiveResult {
    Received(AggregationReceiveResultReceived),
    NoMessage,
}

#[derive(Debug, Constructor, Getters)]
pub struct AggregationReceiveResultReceived {
    #[get = "pub"]
    user_id: UserId,
    #[get = "pub"]
    delete_key: String,
}

#[derive(Debug, Constructor, Getters, Display)]
pub struct AggregationDeleteResult {}

#[async_trait::async_trait]
pub trait AggregationQueue: Interface + Debug + Send + Sync {
    async fn send_message(&self, enqueueable: UserId) -> Result<AggregationSendResult, QueueError>;
    async fn receive_message(&self) -> Result<AggregationReceiveResult, QueueError>;
    async fn delete_message(
        &self,
        receipt_handle: String,
    ) -> Result<AggregationDeleteResult, QueueError>;
}
