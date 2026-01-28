use derive_more::Constructor;

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Constructor)]
pub struct Cell<A> {
    pub value: A,
}
