use crate::repository::repository::RepositoryError;
use rust_job_server_core::model::user::{User, UserId, UserPlan};
use shaku::Interface;
use std::fmt::Debug;

pub trait UserRepository: Debug + Interface + Send + Sync + 'static {
    fn get_user(&self, user: UserId) -> Result<User, RepositoryError>;
    fn get_user_plan(&self, user: UserId) -> Result<UserPlan, RepositoryError>;
}
