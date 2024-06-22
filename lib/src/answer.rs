use crate::{
    val::case::CacheCase,
    Val,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Answer {
    #[default]
    Unsolved,
    Unsolvable,
    Unverified(Val),
    Verified(CacheCase),
}
