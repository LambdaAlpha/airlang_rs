use super::ConstFn;
use super::FreeFn;
use super::MutFn;
use crate::semantics::val::FuncVal;
use crate::type_::ConstRef;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Setup {
    pub(crate) call: Option<FuncVal>,
    pub(crate) solve: Option<FuncVal>,
}

impl Setup {
    pub(crate) fn none() -> Self {
        Self { call: None, solve: None }
    }
}

pub(crate) trait SetupFn {}

impl<T> SetupFn for &T where T: SetupFn {}

impl<I, O, T> FreeFn<I, O> for Option<T>
where
    T: FreeFn<I, O> + SetupFn,
    I: Into<O>,
{
    fn free_call(&self, input: I) -> O {
        match self {
            Some(t) => t.free_call(input),
            None => input.into(),
        }
    }
}

impl<Ctx, I, O, T> ConstFn<Ctx, I, O> for Option<T>
where
    T: ConstFn<Ctx, I, O> + SetupFn,
    Option<T>: FreeFn<I, O>,
    I: Into<O>,
{
    fn const_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        match self {
            Some(t) => t.const_call(ctx, input),
            None => input.into(),
        }
    }
}

impl<Ctx, I, O, T> MutFn<Ctx, I, O> for Option<T>
where
    T: MutFn<Ctx, I, O> + SetupFn,
    Option<T>: ConstFn<Ctx, I, O>,
    I: Into<O>,
{
    fn mut_call(&self, ctx: &mut Ctx, input: I) -> O {
        match self {
            Some(t) => t.mut_call(ctx, input),
            None => input.into(),
        }
    }
}

macro_rules! impl_setup {
    ($func:ty) => {
        impl $crate::semantics::func::FuncSetup for $func {
            fn call(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.call.as_ref()
            }

            fn solve(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.solve.as_ref()
            }
        }
    };
}

pub(crate) use impl_setup;
