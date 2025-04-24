use derive_more::Constructor;
use rust_job_server_application::repository::repository::RepositoryError;
use rust_job_server_application::repository::user_repository::UserRepository;
use rust_job_server_core::model::user::{User, UserId};
use shaku::Component;

#[derive(Debug, Constructor, Component)]
#[shaku(interface = UserRepository)]
pub struct FileUserRepository {}

impl UserRepository for FileUserRepository {
    fn get_user(&self, user: UserId) -> Result<User, RepositoryError> {
        todo!()
    }

    fn get_user_plan(
        &self,
        user: UserId,
    ) -> Result<rust_job_server_core::model::user::UserPlan, RepositoryError> {
        todo!()
    }
}
