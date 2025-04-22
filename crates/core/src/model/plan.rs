use crate::value_object::price::Price;
use derive_more::Constructor;
use getset::Getters;
use uuid::Uuid;

#[derive(Debug, Constructor, Getters)]
pub struct PlanId(Uuid);

#[derive(Debug, Constructor, Getters)]
pub struct Plan {
    #[get = "pub"]
    id: PlanId,
    #[get = "pub"]
    price: Price,
}
