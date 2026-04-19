use derive_more::Constructor;

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Constructor)]
pub struct Quote<A> {
    pub value: A,
}
