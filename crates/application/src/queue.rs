use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::str::FromStr;
use thiserror::Error;

pub mod aggregation_queue;
pub mod user_export_queue;

pub trait Dequeueable: DeserializeOwned + Send + Sync + Debug {}
impl<T> Dequeueable for T where T: DeserializeOwned + Send + Sync + Debug {}

pub trait Enqueueable: Serialize + Send + Sync + Debug {}
impl<T> Enqueueable for T where T: Serialize + Send + Sync + Debug {}

pub trait ReceiptHandleable: AsRef<str> + Send + Sync + Debug {}
impl<T> ReceiptHandleable for T where T: AsRef<str> + Send + Sync + Debug {}

pub enum QueueType {
    Aggregation,
    UserExport,
}

#[derive(Debug, Error)]
pub enum QueueTypeError {
    #[error("Undefined queue type {0}")]
    UndefinedQueueType(String),
}

impl QueueType {
    pub fn queue_name(&self) -> String {
        match self {
            QueueType::Aggregation => "aggregation_queue".to_string(),
            QueueType::UserExport => "user_export_queue".to_string(),
        }
    }
}

impl FromStr for QueueType {
    type Err = QueueTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "aggregation_queue" => Ok(QueueType::Aggregation),
            "user_export_queue" => Ok(QueueType::UserExport),
            _ => Err(QueueTypeError::UndefinedQueueType(s.to_string())),
        }
    }
}
