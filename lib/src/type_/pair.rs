use derive_more::Constructor;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Constructor)]
pub struct Pair<A, B> {
    pub first: A,
    pub second: B,
}
