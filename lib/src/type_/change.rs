use derive_more::Constructor;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Constructor)]
pub struct Change<F, T> {
    pub from: F,
    pub to: T,
}
