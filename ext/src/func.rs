use {
    airlang::{
        CtxForConstFn,
        CtxForMutableFn,
        IoMode,
        Val,
    },
    std::rc::Rc,
};

pub trait ExtCtxFreeFn = Fn(Val) -> Val;

pub trait ExtCtxConstFn = Fn(CtxForConstFn, Val) -> Val;

pub trait ExtCtxMutableFn = Fn(CtxForMutableFn, Val) -> Val;

#[derive(Clone)]
pub struct ExtFunc {
    pub(crate) input_mode: IoMode,
    pub(crate) output_mode: IoMode,
    pub(crate) ext_fn: ExtFn,
}

#[derive(Clone)]
pub enum ExtFn {
    Free(Rc<dyn ExtCtxFreeFn>),
    Const(Rc<dyn ExtCtxConstFn>),
    Mutable(Rc<dyn ExtCtxMutableFn>),
}

impl ExtFn {
    pub fn apply(&self, ctx: CtxForMutableFn, input: Val) -> Val {
        match self {
            ExtFn::Free(f) => f(input),
            ExtFn::Const(f) => f(ctx.to_const(), input),
            ExtFn::Mutable(f) => f(ctx, input),
        }
    }

    pub fn new_free(f: impl ExtCtxFreeFn + 'static) -> Self {
        ExtFn::Free(Rc::new(f))
    }

    pub fn new_const(f: impl ExtCtxConstFn + 'static) -> Self {
        ExtFn::Const(Rc::new(f))
    }

    pub fn new_mutable(f: impl ExtCtxMutableFn + 'static) -> Self {
        ExtFn::Mutable(Rc::new(f))
    }
}

impl ExtFunc {
    pub fn new(input_mode: IoMode, output_mode: IoMode, ext_fn: ExtFn) -> Self {
        Self {
            input_mode,
            output_mode,
            ext_fn,
        }
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
