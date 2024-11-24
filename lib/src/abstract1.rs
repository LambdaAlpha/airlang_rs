use std::hash::Hash;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Abstract<A, B> {
    pub func: A,
    pub input: B,
}

impl<A, B> Abstract<A, B> {
    pub fn new(func: A, input: B) -> Self {
        Self { func, input }
    }
}
