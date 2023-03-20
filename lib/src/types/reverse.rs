use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Reverse<A, B> {
    pub func: A,
    pub output: B,
}

impl<A, B> Reverse<A, B> {
    pub(crate) fn new(func: A, output: B) -> Self {
        Self { func, output }
    }
}
