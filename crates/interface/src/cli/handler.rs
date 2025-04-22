// pub mod example_handler;

pub enum HandlerError {}

pub trait Handler: Send + Sync {
    fn handle(&self) -> Result<(), HandlerError>;
}
