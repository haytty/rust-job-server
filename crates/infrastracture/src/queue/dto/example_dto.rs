use derive_more::Constructor;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Constructor, Getters)]
pub struct ExampleDto {
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    message: String,
}
