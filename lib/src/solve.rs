use std::hash::Hash;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Solve<A> {
    pub func: A,
}

impl<A> Solve<A> {
    pub fn new(func: A) -> Self {
        Self { func }
    }
}
