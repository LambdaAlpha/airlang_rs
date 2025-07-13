#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Call<Func, Ctx, Input> {
    pub reverse: bool,
    pub func: Func,
    pub ctx: Ctx,
    pub input: Input,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FuncInput<Func, Input> {
    pub func: Func,
    pub input: Input,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CtxInput<Ctx, Input> {
    pub ctx: Ctx,
    pub input: Input,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FuncCtx<Func, Ctx> {
    pub func: Func,
    pub ctx: Ctx,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FuncCtxInput<Func, Ctx, Input> {
    pub func: Func,
    pub ctx: Ctx,
    pub input: Input,
}

impl<Func, Ctx, Input> Call<Func, Ctx, Input> {
    pub const fn new(reverse: bool, func: Func, ctx: Ctx, input: Input) -> Self {
        Self { reverse, func, ctx, input }
    }
}

impl<Func, Input> FuncInput<Func, Input> {
    pub const fn new(func: Func, input: Input) -> Self {
        Self { func, input }
    }
}

impl<Ctx, Input> CtxInput<Ctx, Input> {
    pub const fn new(ctx: Ctx, input: Input) -> Self {
        Self { ctx, input }
    }
}

impl<Func, Ctx> FuncCtx<Func, Ctx> {
    pub const fn new(func: Func, ctx: Ctx) -> Self {
        Self { func, ctx }
    }
}

impl<Func, Ctx, Input> FuncCtxInput<Func, Ctx, Input> {
    pub const fn new(func: Func, ctx: Ctx, input: Input) -> Self {
        Self { func, ctx, input }
    }
}
