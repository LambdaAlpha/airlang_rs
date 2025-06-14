#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Pair<A, B> {
    pub first: A,
    pub second: B,
}

impl<A, B> Pair<A, B> {
    pub fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}
