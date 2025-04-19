pub mod register_service;
pub mod server_service;

use anyhow::Result;
use shaku::Interface;

#[async_trait::async_trait]
pub trait Service: Interface {
    async fn run(&self) -> Result<()>;
}
