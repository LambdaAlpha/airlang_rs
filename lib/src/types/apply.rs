use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Apply<A, B> {
    pub func: A,
    pub input: B,
}

impl<A, B> Apply<A, B> {
    pub(crate) fn new(func: A, input: B) -> Self {
        Self { func, input }
    }
}
