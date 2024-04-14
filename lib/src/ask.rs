use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Ask<A, B> {
    pub func: A,
    pub output: B,
}

impl<A, B> Ask<A, B> {
    pub fn new(func: A, output: B) -> Self {
        Self { func, output }
    }
}
