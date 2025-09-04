use derive_more::Constructor;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Constructor)]
pub struct Call<Func, Input> {
    pub func: Func,
    pub input: Input,
}
