use crate::semantics::val::FuncVal;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct FreeSetup {
    pub(crate) call_input: Option<FuncVal>,
    pub(crate) solve_input: Option<FuncVal>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct DynSetup {
    pub(crate) call_ctx: Option<FuncVal>,
    pub(crate) call_input: Option<FuncVal>,
    pub(crate) solve_ctx: Option<FuncVal>,
    pub(crate) solve_input: Option<FuncVal>,
}

impl FreeSetup {
    pub(crate) fn none() -> Self {
        Self { call_input: None, solve_input: None }
    }
}

impl DynSetup {
    pub(crate) fn none() -> Self {
        Self { call_ctx: None, call_input: None, solve_ctx: None, solve_input: None }
    }
}

impl From<DynSetup> for FreeSetup {
    fn from(value: DynSetup) -> Self {
        Self { call_input: value.call_input, solve_input: value.solve_input }
    }
}

impl From<FreeSetup> for DynSetup {
    fn from(value: FreeSetup) -> Self {
        Self {
            call_ctx: None,
            call_input: value.call_input,
            solve_ctx: None,
            solve_input: value.solve_input,
        }
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

macro_rules! impl_free_setup {
    ($func:ty) => {
        impl $crate::semantics::func::FuncSetup for $func {
            fn call_ctx(&self) -> Option<&$crate::semantics::val::FuncVal> {
                None
            }

            fn call_input(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.call_input.as_ref()
            }

            fn solve_ctx(&self) -> Option<&$crate::semantics::val::FuncVal> {
                None
            }

            fn solve_input(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.solve_input.as_ref()
            }
        }
    };
}

pub(crate) use impl_free_setup;

macro_rules! impl_dyn_setup {
    ($func:ty) => {
        impl $crate::semantics::func::FuncSetup for $func {
            fn call_ctx(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.call_ctx.as_ref()
            }

            fn call_input(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.call_input.as_ref()
            }

            fn solve_ctx(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.solve_ctx.as_ref()
            }

            fn solve_input(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.solve_input.as_ref()
            }
        }
    };
}

pub(crate) use impl_dyn_setup;

use crate::semantics::func::ConstCellFn;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeCellFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutCellFn;
use crate::semantics::func::MutStaticFn;
use crate::type_::ConstRef;
