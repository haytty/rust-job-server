use crate::model::plan::Plan;
use derive_more::Constructor;
use getset::Getters;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Invalid id: {0}")]
    InvalidId(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn from_string(string: String) -> Result<Self, UserError> {
        let uuid = Uuid::parse_str(&string).map_err(|_| UserError::InvalidId(string))?;
        Ok(Self::from_uuid(uuid))
    }
}

#[derive(Debug)]
pub struct UserName(String);

#[derive(Debug, Constructor, Getters)]
pub struct User {
    #[get = "pub"]
    id: UserId,
    #[get = "pub"]
    name: UserName,
}

#[derive(Debug, Constructor, Getters)]
pub struct UserPlan {
    #[get = "pub"]
    user: User,
    #[get = "pub"]
    plan: Plan,
}
