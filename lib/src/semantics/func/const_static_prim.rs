use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::Func;
use crate::semantics::func::Setup;
use crate::semantics::val::Val;
use crate::type_::Symbol;
use crate::type_::ref_::ConstRef;

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

impl<Ctx, I, O, T> ConstStaticFn<Ctx, I, O> for &T
where T: ConstStaticFn<Ctx, I, O>
{
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (**self).const_static_call(ctx, input)
    }
}

impl<Ctx, I, O, T> ConstStaticFn<Ctx, I, O> for &mut T
where T: ConstStaticFn<Ctx, I, O>
{
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (**self).const_static_call(ctx, input)
    }
}

#[derive(Clone)]
pub struct ConstStaticPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Rc<dyn ConstStaticFn<Val, Val, Val>>,
    pub(crate) setup: Option<Setup>,
    pub(crate) ctx_explicit: bool,
}

impl FreeStaticFn<Val, Val> for ConstStaticPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn_.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Val, Val> for ConstStaticPrimFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_static_call(ctx, input)
    }
}

impl Func for ConstStaticPrimFunc {
    fn setup(&self) -> Option<&Setup> {
        self.setup.as_ref()
    }

    fn ctx_explicit(&self) -> bool {
        self.ctx_explicit
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
