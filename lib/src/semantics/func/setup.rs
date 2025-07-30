use super::ConstCellFn;
use super::ConstStaticFn;
use super::FreeCellFn;
use super::FreeStaticFn;
use super::MutCellFn;
use super::MutStaticFn;
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

impl<I, O, T> FreeStaticFn<I, O> for Option<T>
where
    T: FreeStaticFn<I, O> + SetupFn,
    I: Into<O>,
{
    fn free_static_call(&self, input: I) -> O {
        match self {
            Some(t) => t.free_static_call(input),
            None => input.into(),
        }
    }
}

impl<I, O, T> FreeCellFn<I, O> for Option<T>
where
    T: FreeCellFn<I, O> + SetupFn,
    I: Into<O>,
{
    fn free_cell_call(&mut self, input: I) -> O {
        match self {
            Some(t) => t.free_cell_call(input),
            None => input.into(),
        }
    }
}

impl<Ctx, I, O, T> ConstStaticFn<Ctx, I, O> for Option<T>
where
    T: ConstStaticFn<Ctx, I, O> + SetupFn,
    Option<T>: FreeStaticFn<I, O>,
    I: Into<O>,
{
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        match self {
            Some(t) => t.const_static_call(ctx, input),
            None => input.into(),
        }
    }
}

impl<Ctx, I, O, T> ConstCellFn<Ctx, I, O> for Option<T>
where
    T: ConstCellFn<Ctx, I, O> + SetupFn,
    Option<T>: FreeCellFn<I, O>,
    I: Into<O>,
{
    fn const_cell_call(&mut self, ctx: ConstRef<Ctx>, input: I) -> O {
        match self {
            Some(t) => t.const_cell_call(ctx, input),
            None => input.into(),
        }
    }
}

impl<Ctx, I, O, T> MutStaticFn<Ctx, I, O> for Option<T>
where
    T: MutStaticFn<Ctx, I, O> + SetupFn,
    Option<T>: ConstStaticFn<Ctx, I, O>,
    I: Into<O>,
{
    fn mut_static_call(&self, ctx: &mut Ctx, input: I) -> O {
        match self {
            Some(t) => t.mut_static_call(ctx, input),
            None => input.into(),
        }
    }
}

impl<Ctx, I, O, T> MutCellFn<Ctx, I, O> for Option<T>
where
    T: MutCellFn<Ctx, I, O> + SetupFn,
    Option<T>: ConstCellFn<Ctx, I, O>,
    I: Into<O>,
{
    fn mut_cell_call(&mut self, ctx: &mut Ctx, input: I) -> O {
        match self {
            Some(t) => t.mut_cell_call(ctx, input),
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
