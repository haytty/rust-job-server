use derive_more::Constructor;
use getset::Getters;
use rust_job_server_interface::job::handler::aggregation::aggregation_handler::AggregationHandleInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Constructor, Getters)]
pub struct AggregationDto {
    #[get = "pub"]
    user_id: String,
}

impl Into<AggregationHandleInput> for AggregationDto {
    fn into(self) -> AggregationHandleInput {
        AggregationHandleInput::new(self.user_id)
    }
}
