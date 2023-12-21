use {
    airlang::{
        CtxForConstFn,
        CtxForMutableFn,
        FuncExt,
        IoMode,
        Symbol,
        Val,
    },
    std::{
        fmt::{
            Debug,
            Formatter,
        },
        hash::{
            Hash,
            Hasher,
        },
    },
};

pub trait ExtCtxFreeFn {
    fn call(&self, input: Val) -> Val;
}

pub trait ExtCtxConstFn {
    fn call(&self, ctx: CtxForConstFn, input: Val) -> Val;
}

pub trait ExtCtxMutableFn {
    fn call(&self, ctx: CtxForMutableFn, input: Val) -> Val;
}

pub struct ExtFunc {
    pub(crate) id: Symbol,
    pub(crate) input_mode: IoMode,
    pub(crate) output_mode: IoMode,
    pub(crate) ext_fn: ExtFn,
}

pub enum ExtFn {
    Free(Box<dyn ExtCtxFreeFn>),
    Const(Box<dyn ExtCtxConstFn>),
    Mutable(Box<dyn ExtCtxMutableFn>),
}

impl ExtFn {
    pub fn call(&self, ctx: CtxForMutableFn, input: Val) -> Val {
        match self {
            ExtFn::Free(f) => f.call(input),
            ExtFn::Const(f) => f.call(ctx.to_const(), input),
            ExtFn::Mutable(f) => f.call(ctx, input),
        }
    }

    pub fn new_free(f: impl ExtCtxFreeFn + 'static) -> Self {
        ExtFn::Free(Box::new(f))
    }

    pub fn new_const(f: impl ExtCtxConstFn + 'static) -> Self {
        ExtFn::Const(Box::new(f))
    }

    pub fn new_mutable(f: impl ExtCtxMutableFn + 'static) -> Self {
        ExtFn::Mutable(Box::new(f))
    }
}

impl ExtFunc {
    pub fn new(id: Symbol, input_mode: IoMode, output_mode: IoMode, ext_fn: ExtFn) -> Self {
        Self {
            id,
            input_mode,
            output_mode,
            ext_fn,
        }
    }

    pub fn id(&self) -> &Symbol {
        &self.id
    }

    pub fn input_mode(&self) -> &IoMode {
        &self.input_mode
    }

    pub fn output_mode(&self) -> &IoMode {
        &self.output_mode
    }

    pub fn ext_fn(&self) -> &ExtFn {
        &self.ext_fn
    }
}

impl<T> ExtCtxFreeFn for T
where
    T: Fn(Val) -> Val,
{
    fn call(&self, input: Val) -> Val {
        self(input)
    }
}

impl<T> ExtCtxConstFn for T
where
    T: Fn(CtxForConstFn, Val) -> Val,
{
    fn call(&self, ctx: CtxForConstFn, input: Val) -> Val {
        self(ctx, input)
    }
}

impl<T> ExtCtxMutableFn for T
where
    T: Fn(CtxForMutableFn, Val) -> Val,
{
    fn call(&self, ctx: CtxForMutableFn, input: Val) -> Val {
        self(ctx, input)
    }
}

impl FuncExt for ExtFunc {
    fn input_mode(&self) -> &IoMode {
        &self.input_mode
    }

    fn output_mode(&self) -> &IoMode {
        &self.output_mode
    }

    fn call(&self, ctx: CtxForMutableFn, input: Val) -> Val {
        self.ext_fn.call(ctx, input)
    }
}

impl Debug for ExtFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtFunc")
            .field("id", &self.id)
            .field("input_mode", &self.input_mode)
            .field("output_mode", &self.output_mode)
            .finish()
    }
}

impl PartialEq for ExtFunc {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ExtFunc {}

impl Hash for ExtFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
