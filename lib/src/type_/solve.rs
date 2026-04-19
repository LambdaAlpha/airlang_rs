use derive_more::Constructor;

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Constructor)]
pub struct Solve<Func, Output> {
    pub func: Func,
    pub output: Output,
}
