use std::rc::Rc;

use super::ConstFn;
use super::FreeFn;
use super::MutFn;
use super::MutPrimFunc;
use crate::semantics::core::Eval;
use crate::semantics::val::FuncVal;
use crate::type_::ConstRef;
use crate::type_::Symbol;

pub(crate) trait Setup {
    fn setup(&self) -> Option<&FuncVal>;
}

pub(crate) trait SetupFn {}

pub fn default_setup() -> FuncVal {
    FuncVal::MutPrim(
        MutPrimFunc {
            id: Symbol::from_str_unchecked("setup.default"),
            fn_: Rc::new(Eval),
            setup: None,
        }
        .into(),
    )
}

impl<T> SetupFn for &T where T: SetupFn {}

impl<Cfg, I, O, T> FreeFn<Cfg, I, O> for Option<T>
where
    T: FreeFn<Cfg, I, O> + SetupFn,
    I: Into<O>,
{
    fn free_call(&self, cfg: &mut Cfg, input: I) -> O {
        match self {
            Some(t) => t.free_call(cfg, input),
            None => input.into(),
        }
    }
}

impl<Cfg, Ctx, I, O, T> ConstFn<Cfg, Ctx, I, O> for Option<T>
where
    T: ConstFn<Cfg, Ctx, I, O> + SetupFn,
    Option<T>: FreeFn<Cfg, I, O>,
    I: Into<O>,
{
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Ctx>, input: I) -> O {
        match self {
            Some(t) => t.const_call(cfg, ctx, input),
            None => input.into(),
        }
    }
}

impl<Cfg, Ctx, I, O, T> MutFn<Cfg, Ctx, I, O> for Option<T>
where
    T: MutFn<Cfg, Ctx, I, O> + SetupFn,
    Option<T>: ConstFn<Cfg, Ctx, I, O>,
    I: Into<O>,
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, input: I) -> O {
        match self {
            Some(t) => t.mut_call(cfg, ctx, input),
            None => input.into(),
        }
    }
}
