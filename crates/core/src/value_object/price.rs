use derive_more::Constructor;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone, Copy, Hash, Constructor)]
pub struct Price(u64);
