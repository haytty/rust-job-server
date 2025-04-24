use crate::repository::user_repository::UserRepository;
use crate::usecase::aggregation::aggregation_input::AggregationInput;
use crate::usecase::aggregation::aggregation_output::AggregationOutput;
use crate::usecase::aggregation::aggregation_usecase::{
    AggregationUseCase, AggregationUseCaseError,
};
use derive_more::Constructor;
use shaku::Component;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Constructor, Component)]
#[shaku(interface = AggregationUseCase)]
pub struct AggregationFromFileInteractor {
    #[shaku(inject)]
    repository: Arc<dyn UserRepository>,
}

#[async_trait::async_trait]
impl AggregationUseCase for AggregationFromFileInteractor {
    async fn apply(
        &self,
        input: AggregationInput,
    ) -> Result<AggregationOutput, AggregationUseCaseError> {
        info!("Aggregation from file!!!");

        Ok(AggregationOutput::new())
    }
}
