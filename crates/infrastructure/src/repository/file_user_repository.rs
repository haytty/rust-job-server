use derive_more::Constructor;
use rust_job_server_application::repository::user_repository::{
    UserRepository, UserRepositoryError,
};
use rust_job_server_core::model::user::{User, UserId};

#[derive(Debug, Constructor)]
pub struct FileUserRepository {}

impl UserRepository for FileUserRepository {
    fn get_user(&self, user: UserId) -> Result<User, UserRepositoryError> {
        todo!()
    }

    fn get_user_plan(
        &self,
        user: UserId,
    ) -> Result<rust_job_server_core::model::user::UserPlan, UserRepositoryError> {
        todo!()
    }
}
