use derive_more::{Constructor, Display};
use getset::Getters;
use rust_job_server_core::model::user::UserId;
use shaku::Interface;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserExportQueueError {
    #[error("Send Error {0}")]
    SendError(String),
    #[error("Receive Error {0}")]
    ReceiveError(String),
    #[error("Delete Error {0}")]
    DeleteError(String),
}

#[derive(Debug, Constructor, Getters, Display)]
pub struct UserExportSendResult {}

#[derive(Debug)]
pub enum UserExportReceiveResult {
    Received(UserExportReceiveResultReceived),
    NoMessage,
}

#[derive(Debug, Constructor, Getters)]
pub struct UserExportReceiveResultReceived {
    #[get = "pub"]
    user_id: UserId,
    #[get = "pub"]
    delete_key: String,
}

#[derive(Debug, Constructor, Getters, Display)]
pub struct UserExportDeleteResult {}

#[async_trait::async_trait]
pub trait UserExportQueue: Interface + Send + Sync + 'static + Debug {
    async fn send_message(
        &self,
        enqueueable: UserId,
    ) -> Result<UserExportSendResult, UserExportQueueError>;
    async fn receive_message(&self) -> Result<UserExportReceiveResult, UserExportQueueError>;
    async fn delete_message(
        &self,
        receipt_handle: String,
    ) -> Result<UserExportDeleteResult, UserExportQueueError>;
}
