use std::hash::Hash;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Inverse<A> {
    pub func: A,
}

impl<A> Inverse<A> {
    pub fn new(func: A) -> Self {
        Self { func }
    }
}
