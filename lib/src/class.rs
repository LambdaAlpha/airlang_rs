use std::hash::Hash;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Class<A> {
    pub func: A,
}

impl<A> Class<A> {
    pub fn new(func: A) -> Self {
        Self { func }
    }
}
