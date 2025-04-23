use derive_more::Constructor;
use getset::Getters;
use rust_job_server_interface::job::handler::user_export::user_export_handler::UserExportHandleInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Constructor, Getters)]
pub struct UserExportDto {
    #[get = "pub"]
    user_id: String,
}

impl Into<UserExportHandleInput> for UserExportDto {
    fn into(self) -> UserExportHandleInput {
        UserExportHandleInput::new(self.user_id)
    }
}
