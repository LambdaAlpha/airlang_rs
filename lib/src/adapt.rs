use std::hash::Hash;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Adapt<A, B> {
    pub spec: A,
    pub value: B,
}

impl<A, B> Adapt<A, B> {
    pub fn new(spec: A, value: B) -> Self {
        Self { spec, value }
    }
}
