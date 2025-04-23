use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub mod aggregation_queue;
pub mod user_export_queue;

pub trait Dequeueable: DeserializeOwned + Send + Sync + Debug {}
impl<T> Dequeueable for T where T: DeserializeOwned + Send + Sync + Debug {}

pub trait Enqueueable: Serialize + Send + Sync + Debug {}
impl<T> Enqueueable for T where T: Serialize + Send + Sync + Debug {}

pub trait ReceiptHandleable: AsRef<str> + Send + Sync + Debug {}
impl<T> ReceiptHandleable for T where T: AsRef<str> + Send + Sync + Debug {}
