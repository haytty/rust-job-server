use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait Dequeueable: DeserializeOwned + Send + Sync + Debug {}
impl<T> Dequeueable for T where T: DeserializeOwned + Send + Sync + Debug {}

pub trait Enqueueable: Serialize + Send + Sync + Debug {}
impl<T> Enqueueable for T where T: Serialize + Send + Sync + Debug {}

pub trait ReceiptHandleable: AsRef<str> + Send + Sync + Debug {}
impl<T> ReceiptHandleable for T where T: AsRef<str> + Send + Sync + Debug {}

use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueueError {
    #[error("Send Error {0}")]
    SendError(String),
    #[error("Receive Error {0}")]
    ReceiveError(String),
    #[error("Delete Error {0}")]
    DeleteError(String),
}

#[derive(Debug)]
pub enum QueueType {
    Aggregation,
    UserExport,
}

#[derive(Debug, Error)]
pub enum QueueTypeError {
    #[error("Undefined queue type {0}")]
    UndefinedQueueType(String),
}

pub const AGGREGATION_QUEUE_NAME: &str = "aggregation_queue";
pub const USER_EXPORT_QUEUE_NAME: &str = "user_export_queue";

impl QueueType {
    pub fn queue_name(&self) -> String {
        match self {
            QueueType::Aggregation => AGGREGATION_QUEUE_NAME.to_string(),
            QueueType::UserExport => USER_EXPORT_QUEUE_NAME.to_string(),
        }
    }
}

impl FromStr for QueueType {
    type Err = QueueTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            AGGREGATION_QUEUE_NAME => Ok(QueueType::Aggregation),
            USER_EXPORT_QUEUE_NAME => Ok(QueueType::UserExport),
            _ => Err(QueueTypeError::UndefinedQueueType(s.to_string())),
        }
    }
}
