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
