use std::hash::Hash;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Call<A, B> {
    pub func: A,
    pub input: B,
}

impl<A, B> Call<A, B> {
    pub fn new(func: A, input: B) -> Self {
        Self { func, input }
    }
}
