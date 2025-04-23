pub mod aggregation;
pub mod user_export;

pub trait HandlerError {}

#[async_trait::async_trait]
pub trait Handler<I, O, E>: Send + Sync {
    async fn handle(&self, handle_input: I) -> Result<O, E>;
}

pub trait HandleInput<UI, E>: Send + Sync {
    fn to_use_case_input(self) -> Result<UI, E>;
}

pub trait HandleOutput<UO, E>: Send + Sync + Sized {
    fn from_use_case_output(use_case_output: UO) -> Result<Self, E>;
}
