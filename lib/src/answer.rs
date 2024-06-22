use crate::{
    val::case::CacheCase,
    Val,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Answer {
    #[default]
    None,
    Never,
    Maybe(Val),
    Cache(CacheCase),
}

pub(crate) const NONE: &str = "none";
pub(crate) const NEVER: &str = "never";
pub(crate) const MAYBE: &str = "maybe";
pub(crate) const CACHE: &str = "cache";
