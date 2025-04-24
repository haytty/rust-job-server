use std::fmt::Debug;
use shaku::Interface;
use rust_job_server_core::model::user::{User, UserId, UserPlan};

pub struct UserRepositoryError {}

pub trait UserRepository: Debug + Interface + Send + Sync + 'static {
    fn get_user(&self, user: UserId) -> Result<User, UserRepositoryError>;
    fn get_user_plan(&self, user: UserId) -> Result<UserPlan, UserRepositoryError>;
}
