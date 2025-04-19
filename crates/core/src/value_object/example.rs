use derive_more::Constructor;
use getset::Getters;

#[derive(Debug, Constructor, Getters)]
pub struct Example {
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    message: String,
}
