use derive_more::Constructor;
use getset::Getters;
use rust_job_server_core::model::user::{UserError, UserId};
use thiserror::Error;

#[derive(Debug, Constructor, Getters)]
pub struct UserExportInput {
    #[get = "pub"]
    user_id: UserId,
}

#[derive(Debug, Error)]
pub enum UserExportInputError {
    #[error("Invalid user id: {0}")]
    InvalidUserId(UserError),
}

impl UserExportInput {
    pub fn from_user_id_string(user_id: String) -> Result<Self, UserExportInputError> {
        let uuid =
            UserId::from_string(user_id).map_err(|e| UserExportInputError::InvalidUserId(e))?;

        Ok(Self::new(uuid))
    }
}
