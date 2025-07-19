use derive_more::Constructor;
use derive_more::IsVariant;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Task<Func, Ctx, Input> {
    pub action: Action,
    pub func: Func,
    pub ctx: Ctx,
    pub input: Input,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, IsVariant)]
pub enum Action {
    #[default]
    Call,
    Solve,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Constructor)]
pub struct FuncInput<Func, Input> {
    pub func: Func,
    pub input: Input,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Constructor)]
pub struct CtxInput<Ctx, Input> {
    pub ctx: Ctx,
    pub input: Input,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Constructor)]
pub struct FuncCtx<Func, Ctx> {
    pub func: Func,
    pub ctx: Ctx,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Constructor)]
pub struct FuncCtxInput<Func, Ctx, Input> {
    pub func: Func,
    pub ctx: Ctx,
    pub input: Input,
}
