use super::CodeMode;
use super::Mode;
use super::ModeFn;
use super::PrimMode;
use crate::semantics::core::TaskEval;
use crate::semantics::core::TaskForm;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::TaskVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::CtxInput;
use crate::type_::FuncCtxInput;
use crate::type_::Map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskMode {
    pub func: Option<Mode>,
    pub ctx: Option<Mode>,
    pub input: Option<Mode>,
    pub some: Option<TaskMapMode>,
}

pub type TaskMapMode = Map<Val, CtxInput<Option<Mode>, Option<Mode>>>;

impl ModeFn for TaskMode {}

impl FreeStaticFn<TaskVal, Val> for TaskMode {
    fn free_static_call(&self, input: TaskVal) -> Val {
        match &self.some {
            Some(some) => {
                let else_ = FuncCtxInput { func: &self.func, ctx: &self.ctx, input: &self.input };
                TaskForm { some, else_ }.free_static_call(input)
            }
            None => TaskEval { func: &self.func, ctx: &self.ctx, input: &self.input }
                .free_static_call(input),
        }
    }
}

impl ConstStaticFn<Val, TaskVal, Val> for TaskMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: TaskVal) -> Val {
        match &self.some {
            Some(some) => {
                let else_ = FuncCtxInput { func: &self.func, ctx: &self.ctx, input: &self.input };
                TaskForm { some, else_ }.const_static_call(ctx, input)
            }
            None => TaskEval { func: &self.func, ctx: &self.ctx, input: &self.input }
                .const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, TaskVal, Val> for TaskMode {
    fn mut_static_call(&self, ctx: &mut Val, input: TaskVal) -> Val {
        match &self.some {
            Some(some) => {
                let else_ = FuncCtxInput { func: &self.func, ctx: &self.ctx, input: &self.input };
                TaskForm { some, else_ }.mut_static_call(ctx, input)
            }
            None => TaskEval { func: &self.func, ctx: &self.ctx, input: &self.input }
                .mut_static_call(ctx, input),
        }
    }
}

impl TryFrom<PrimMode> for TaskMode {
    type Error = ();

    fn try_from(mode: PrimMode) -> Result<Self, Self::Error> {
        let Some(code) = mode.task else {
            return Err(());
        };
        let some = match code {
            CodeMode::Form => Some(Map::default()),
            CodeMode::Eval => None,
        };
        Ok(Self { some, func: Some(mode.into()), ctx: Some(mode.into()), input: Some(mode.into()) })
    }
}
