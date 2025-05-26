use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::Symbol;
use crate::Val;
use crate::either::Either;
use crate::func::FuncTrait;
use crate::func::prim::Primitive;
use crate::types::ref1::DynRef;

pub trait MutStaticFn<Ctx, I, O>: ConstStaticFn<Ctx, I, O> {
    fn mut_static_call(&self, ctx: &mut Ctx, input: I) -> O {
        self.const_static_call(ConstRef::new(ctx), input)
    }

    fn dyn_static_call(&self, ctx: DynRef<Ctx>, input: I) -> O {
        match ctx.into_either() {
            Either::This(ctx) => self.const_static_call(ctx, input),
            Either::That(ctx) => self.mut_static_call(ctx, input),
        }
    }

    fn opt_dyn_static_call(&self, ctx: Option<DynRef<Ctx>>, input: I) -> O {
        match ctx {
            Some(ctx) => self.dyn_static_call(ctx, input),
            None => self.free_static_call(input),
        }
    }
}

pub struct MutStaticImpl<Ctx, I, O> {
    pub free: fn(I) -> O,
    pub const1: fn(ConstRef<Ctx>, I) -> O,
    pub mut1: fn(&mut Ctx, I) -> O,
}

#[derive(Clone)]
pub struct MutStaticPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Rc<dyn MutStaticFn<Ctx, Val, Val>>,
    pub(crate) mode: FuncMode,
}

impl FreeStaticFn<Val, Val> for MutStaticPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn1.free_static_call(input)
    }
}

impl ConstStaticFn<Ctx, Val, Val> for MutStaticPrimFunc {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: Val) -> Val {
        self.fn1.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Ctx, Val, Val> for MutStaticPrimFunc {
    fn mut_static_call(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.fn1.mut_static_call(ctx, input)
    }
}

impl FuncTrait for MutStaticPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn code(&self) -> Val {
        Val::default()
    }
}

impl MutStaticPrimFunc {
    pub fn new_extension(
        id: Symbol, fn1: Rc<dyn MutStaticFn<Ctx, Val, Val>>, mode: FuncMode,
    ) -> Self {
        Self { prim: Primitive { id, is_extension: true }, fn1, mode }
    }

    pub(crate) fn new(id: Symbol, fn1: Rc<dyn MutStaticFn<Ctx, Val, Val>>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: false }, fn1, mode }
    }
}

impl Debug for MutStaticPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.prim.fmt(f)
    }
}

impl PartialEq for MutStaticPrimFunc {
    fn eq(&self, other: &MutStaticPrimFunc) -> bool {
        self.prim == other.prim
    }
}

impl Eq for MutStaticPrimFunc {}

impl Hash for MutStaticPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prim.hash(state);
    }
}

impl<Ctx, I, O> FreeStaticFn<I, O> for MutStaticImpl<Ctx, I, O> {
    fn free_static_call(&self, input: I) -> O {
        (self.free)(input)
    }
}

impl<Ctx, I, O> ConstStaticFn<Ctx, I, O> for MutStaticImpl<Ctx, I, O> {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (self.const1)(ctx, input)
    }
}

impl<Ctx, I, O> MutStaticFn<Ctx, I, O> for MutStaticImpl<Ctx, I, O> {
    fn mut_static_call(&self, ctx: &mut Ctx, input: I) -> O {
        (self.mut1)(ctx, input)
    }
}

impl<Ctx, I, O> MutStaticImpl<Ctx, I, O> {
    pub fn new(
        free: fn(I) -> O, const1: fn(ConstRef<Ctx>, I) -> O, mut1: fn(&mut Ctx, I) -> O,
    ) -> Self {
        Self { free, const1, mut1 }
    }

    pub fn default(_ctx: DynRef<Ctx>, _input: I) -> O
    where O: Default {
        O::default()
    }
}
