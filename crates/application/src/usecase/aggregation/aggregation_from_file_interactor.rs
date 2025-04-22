use crate::repository::user_repository::UserRepository;
use crate::usecase::aggregation::aggregation_input::AggregationInput;
use crate::usecase::aggregation::aggregation_output::AggregationOutput;
use crate::usecase::aggregation::aggregation_usecase::{AggregationError, AggregationUseCase};
use derive_more::Constructor;
use getset::Getters;
use tracing::info;

#[derive(Debug, Constructor)]
pub struct AggregationFromFileInteractor<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> AggregationUseCase for AggregationFromFileInteractor<R>
where
    R: UserRepository,
{
    fn apply(&self, input: AggregationInput) -> Result<AggregationOutput, AggregationError> {
        info!("Aggregation from file!!!");

        Ok(AggregationOutput::new())
    }
}
