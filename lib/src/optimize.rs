use std::hash::Hash;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Optimize<A> {
    pub func: A,
}

impl<A> Optimize<A> {
    pub fn new(func: A) -> Self {
        Self { func }
    }
}
