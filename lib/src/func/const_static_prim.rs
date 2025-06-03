use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use crate::FreeStaticFn;
use crate::FuncMode;
use crate::Symbol;
use crate::Val;
use crate::func::FuncTrait;
use crate::types::ref1::ConstRef;

pub trait ConstStaticFn<Ctx, I, O>: FreeStaticFn<I, O> {
    #[allow(unused_variables)]
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        self.free_static_call(input)
    }

    fn opt_const_static_call(&self, ctx: Option<ConstRef<Ctx>>, input: I) -> O {
        match ctx {
            Some(ctx) => self.const_static_call(ctx, input),
            None => self.free_static_call(input),
        }
    }
}

pub struct ConstStaticImpl<Ctx, I, O> {
    pub free: fn(I) -> O,
    pub const1: fn(ConstRef<Ctx>, I) -> O,
}

#[derive(Clone)]
pub struct ConstStaticPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) extension: bool,
    pub(crate) fn1: Rc<dyn ConstStaticFn<Val, Val, Val>>,
    pub(crate) mode: FuncMode,
    pub(crate) ctx_explicit: bool,
}

impl FreeStaticFn<Val, Val> for ConstStaticPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn1.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Val, Val> for ConstStaticPrimFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn1.const_static_call(ctx, input)
    }
}

impl FuncTrait for ConstStaticPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn ctx_explicit(&self) -> bool {
        self.ctx_explicit
    }

    fn code(&self) -> Val {
        Val::default()
    }
}

impl ConstStaticPrimFunc {
    pub fn new(
        id: Symbol, fn1: Rc<dyn ConstStaticFn<Val, Val, Val>>, mode: FuncMode, ctx_explicit: bool,
    ) -> Self {
        Self { id, extension: true, fn1, mode, ctx_explicit }
    }
}

impl Debug for ConstStaticPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl PartialEq for ConstStaticPrimFunc {
    fn eq(&self, other: &ConstStaticPrimFunc) -> bool {
        self.id == other.id
    }
}

impl Eq for ConstStaticPrimFunc {}

impl Hash for ConstStaticPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<Ctx, I, O> FreeStaticFn<I, O> for ConstStaticImpl<Ctx, I, O> {
    fn free_static_call(&self, input: I) -> O {
        (self.free)(input)
    }
}

impl<Ctx, I, O> ConstStaticFn<Ctx, I, O> for ConstStaticImpl<Ctx, I, O> {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (self.const1)(ctx, input)
    }
}

impl<Ctx, I, O> ConstStaticImpl<Ctx, I, O> {
    pub fn new(free: fn(I) -> O, const1: fn(ConstRef<Ctx>, I) -> O) -> Self {
        Self { free, const1 }
    }

    pub fn default(_ctx: ConstRef<Ctx>, _input: I) -> O
    where O: Default {
        O::default()
    }
}
