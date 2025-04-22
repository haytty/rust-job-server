use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub mod dto;
pub mod sqs_queue;

pub trait Dequeueable: DeserializeOwned + Send + Sync + Debug {}
impl<T> Dequeueable for T where T: DeserializeOwned + Send + Sync + Debug {}
