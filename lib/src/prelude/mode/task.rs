use super::Mode;
use crate::semantics::core::TaskEval;
use crate::semantics::core::TaskForm;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskMode {
    pub func: Mode,
    pub ctx: Mode,
    pub input: Mode,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TaskPrimMode {
    Form,
    Eval,
}

impl TaskMode {
    pub(crate) fn form(&self) -> TaskForm<'_, Mode, Mode, Mode> {
        TaskForm { func: &self.func, ctx: &self.ctx, input: &self.input }
    }

    pub(crate) fn eval(&self) -> TaskEval<'_, Mode> {
        TaskEval { func: &self.func }
    }
}
