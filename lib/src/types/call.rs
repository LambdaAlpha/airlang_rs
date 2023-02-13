use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Call<A, B> {
    pub(crate) func: A,
    pub(crate) arg: B,
}

impl<A, B> Call<A, B> {
    pub(crate) fn new(func: A, arg: B) -> Self {
        Self { func, arg }
    }
}
