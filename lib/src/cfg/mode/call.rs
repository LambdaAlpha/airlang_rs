use super::Mode;
use crate::semantics::core::CallEval;
use crate::semantics::core::CallForm;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallMode {
    pub func: Mode,
    pub input: Mode,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CallPrimMode {
    Form,
    Eval,
}

impl CallMode {
    pub(crate) fn form(&self) -> CallForm<'_, Mode, Mode> {
        CallForm { func: &self.func, input: &self.input }
    }

    pub(crate) fn eval(&self) -> CallEval<'_, Mode> {
        CallEval { func: &self.func }
    }
}
