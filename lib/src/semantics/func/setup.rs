use crate::semantics::val::FuncVal;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct FreeSetup {
    pub(crate) forward_input: Option<FuncVal>,
    pub(crate) reverse_input: Option<FuncVal>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct DynSetup {
    pub(crate) forward_ctx: Option<FuncVal>,
    pub(crate) forward_input: Option<FuncVal>,
    pub(crate) reverse_ctx: Option<FuncVal>,
    pub(crate) reverse_input: Option<FuncVal>,
}

impl FreeSetup {
    pub(crate) fn none() -> Self {
        Self { forward_input: None, reverse_input: None }
    }
}

impl DynSetup {
    pub(crate) fn none() -> Self {
        Self { forward_ctx: None, forward_input: None, reverse_ctx: None, reverse_input: None }
    }
}

impl From<DynSetup> for FreeSetup {
    fn from(value: DynSetup) -> Self {
        Self { forward_input: value.forward_input, reverse_input: value.reverse_input }
    }
}

impl From<FreeSetup> for DynSetup {
    fn from(value: FreeSetup) -> Self {
        Self {
            forward_ctx: None,
            forward_input: value.forward_input,
            reverse_ctx: None,
            reverse_input: value.reverse_input,
        }
    }
}

macro_rules! impl_free_setup {
    ($func:ty) => {
        impl $crate::semantics::func::FuncSetup for $func {
            fn forward_ctx(&self) -> Option<&$crate::semantics::val::FuncVal> {
                None
            }

            fn forward_input(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.forward_input.as_ref()
            }

            fn reverse_ctx(&self) -> Option<&$crate::semantics::val::FuncVal> {
                None
            }

            fn reverse_input(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.reverse_input.as_ref()
            }
        }
    };
}

pub(crate) use impl_free_setup;

macro_rules! impl_dyn_setup {
    ($func:ty) => {
        impl $crate::semantics::func::FuncSetup for $func {
            fn forward_ctx(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.forward_ctx.as_ref()
            }

            fn forward_input(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.forward_input.as_ref()
            }

            fn reverse_ctx(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.reverse_ctx.as_ref()
            }

            fn reverse_input(&self) -> Option<&$crate::semantics::val::FuncVal> {
                self.setup.reverse_input.as_ref()
            }
        }
    };
}

pub(crate) use impl_dyn_setup;
