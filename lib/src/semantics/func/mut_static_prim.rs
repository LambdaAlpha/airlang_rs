use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::FuncMode;
use crate::semantics::func::FuncTrait;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Either;
use crate::type_::Symbol;
use crate::type_::ref_::DynRef;

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
    pub const_: fn(ConstRef<Ctx>, I) -> O,
    pub mut_: fn(&mut Ctx, I) -> O,
}

#[derive(Clone)]
pub struct MutStaticPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Rc<dyn MutStaticFn<Val, Val, Val>>,
    pub(crate) mode: FuncMode,
    pub(crate) ctx_explicit: bool,
}

impl FreeStaticFn<Val, Val> for MutStaticPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn_.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Val, Val> for MutStaticPrimFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, Val, Val> for MutStaticPrimFunc {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        self.fn_.mut_static_call(ctx, input)
    }
}

impl FuncTrait for MutStaticPrimFunc {
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

impl MutStaticPrimFunc {
    pub fn new(
        id: Symbol, fn_: Rc<dyn MutStaticFn<Val, Val, Val>>, mode: FuncMode, ctx_explicit: bool,
    ) -> Self {
        Self { id, fn_, mode, ctx_explicit }
    }
}

impl Debug for MutStaticPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl PartialEq for MutStaticPrimFunc {
    fn eq(&self, other: &MutStaticPrimFunc) -> bool {
        self.id == other.id
    }
}

impl Eq for MutStaticPrimFunc {}

impl Hash for MutStaticPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<Ctx, I, O> FreeStaticFn<I, O> for MutStaticImpl<Ctx, I, O> {
    fn free_static_call(&self, input: I) -> O {
        (self.free)(input)
    }
}

impl<Ctx, I, O> ConstStaticFn<Ctx, I, O> for MutStaticImpl<Ctx, I, O> {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (self.const_)(ctx, input)
    }
}

impl<Ctx, I, O> MutStaticFn<Ctx, I, O> for MutStaticImpl<Ctx, I, O> {
    fn mut_static_call(&self, ctx: &mut Ctx, input: I) -> O {
        (self.mut_)(ctx, input)
    }
}

impl<Ctx, I, O> MutStaticImpl<Ctx, I, O> {
    pub fn new(
        free: fn(I) -> O, const_: fn(ConstRef<Ctx>, I) -> O, mut_: fn(&mut Ctx, I) -> O,
    ) -> Self {
        Self { free, const_, mut_ }
    }

    pub fn default(_ctx: DynRef<Ctx>, _input: I) -> O
    where O: Default {
        O::default()
    }
}
