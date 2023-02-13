use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Pair<A, B> {
    pub(crate) first: A,
    pub(crate) second: B,
}

impl<A, B> Pair<A, B> {
    pub(crate) fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}
