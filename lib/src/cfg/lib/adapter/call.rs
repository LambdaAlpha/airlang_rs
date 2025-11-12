use crate::cfg::lib::adapter::CoreAdapter;
use crate::semantics::core::CallEval;
use crate::semantics::core::CallForm;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallAdapter {
    pub func: CoreAdapter,
    pub input: CoreAdapter,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CallPrimAdapter {
    Data,
    Code,
}

impl CallAdapter {
    pub(crate) fn form(&self) -> CallForm<'_, CoreAdapter, CoreAdapter> {
        CallForm { func: &self.func, input: &self.input }
    }

    pub(crate) fn eval(&self) -> CallEval<'_, CoreAdapter> {
        CallEval { func: &self.func }
    }
}
