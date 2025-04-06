use std::hash::Hash;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Equiv<A> {
    pub func: A,
}

impl<A> Equiv<A> {
    pub fn new(func: A) -> Self {
        Self { func }
    }
}
