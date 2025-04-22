use derive_more::Constructor;
use getset::Getters;
use rust_job_server_core::model::user::{UserError, UserId};
use thiserror::Error;

#[derive(Debug, Constructor, Getters)]
pub struct AggregationInput {
    user_id: UserId,
}

#[derive(Debug, Error)]
pub enum AggregationInputError {
    #[error("Invalid user id: {0}")]
    InvalidUserId(UserError),
}

impl AggregationInput {
    pub fn from_user_id_string(user_id: String) -> Result<Self, AggregationInputError> {
        let uuid =
            UserId::from_string(user_id).map_err(|e| AggregationInputError::InvalidUserId(e))?;

        Ok(Self::new(uuid))
    }
}
