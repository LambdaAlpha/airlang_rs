use std::hash::Hash;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Comment<A, B> {
    pub meta: A,
    pub value: B,
}

impl<A, B> Comment<A, B> {
    pub fn new(meta: A, value: B) -> Self {
        Self { meta, value }
    }
}
