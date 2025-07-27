use super::CodeMode;
use super::Mode;
use super::PrimMode;
use crate::semantics::core::TaskEval;
use crate::semantics::core::TaskForm;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::func::SetupFn;
use crate::semantics::val::TaskVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskMode {
    pub code: CodeMode,
    pub func: Option<Mode>,
    pub ctx: Option<Mode>,
    pub input: Option<Mode>,
}

impl SetupFn for TaskMode {}

impl FreeStaticFn<TaskVal, Val> for TaskMode {
    fn free_static_call(&self, input: TaskVal) -> Val {
        match self.code {
            CodeMode::Form => TaskForm { func: &self.func, ctx: &self.ctx, input: &self.input }
                .free_static_call(input),
            CodeMode::Eval => TaskEval { func: &self.func }.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Val, TaskVal, Val> for TaskMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: TaskVal) -> Val {
        match self.code {
            CodeMode::Form => TaskForm { func: &self.func, ctx: &self.ctx, input: &self.input }
                .const_static_call(ctx, input),
            CodeMode::Eval => TaskEval { func: &self.func }.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, TaskVal, Val> for TaskMode {
    fn mut_static_call(&self, ctx: &mut Val, input: TaskVal) -> Val {
        match self.code {
            CodeMode::Form => TaskForm { func: &self.func, ctx: &self.ctx, input: &self.input }
                .mut_static_call(ctx, input),
            CodeMode::Eval => TaskEval { func: &self.func }.mut_static_call(ctx, input),
        }
    }
}

impl TryFrom<PrimMode> for TaskMode {
    type Error = ();

    fn try_from(mode: PrimMode) -> Result<Self, Self::Error> {
        let Some(code) = mode.task else {
            return Err(());
        };
        Ok(Self { code, func: Some(mode.into()), ctx: Some(mode.into()), input: Some(mode.into()) })
    }
}
