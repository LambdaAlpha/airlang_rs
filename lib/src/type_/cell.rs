use derive_more::Constructor;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Constructor)]
pub struct Cell<A> {
    pub value: A,
}
