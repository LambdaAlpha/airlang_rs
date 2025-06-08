use crate::semantics::core::CallEval;
use crate::semantics::core::CallForm;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::mode::CodeMode;
use crate::semantics::mode::Mode;
use crate::semantics::mode::ModeFn;
use crate::semantics::mode::PrimMode;
use crate::semantics::val::CallVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallMode {
    pub func: Option<Mode>,
    pub input: Option<Mode>,
    pub some: Option<Map<Val, Option<Mode>>>,
}

impl ModeFn for CallMode {}

impl FreeStaticFn<CallVal, Val> for CallMode {
    fn free_static_call(&self, input: CallVal) -> Val {
        match &self.some {
            Some(some) => {
                CallForm { func: &self.func, input: &self.input, some }.free_static_call(input)
            }
            None => CallEval { func: &self.func, input: &self.input }.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Val, CallVal, Val> for CallMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: CallVal) -> Val {
        match &self.some {
            Some(some) => CallForm { func: &self.func, input: &self.input, some }
                .const_static_call(ctx, input),
            None => CallEval { func: &self.func, input: &self.input }.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, CallVal, Val> for CallMode {
    fn mut_static_call(&self, ctx: &mut Val, input: CallVal) -> Val {
        match &self.some {
            Some(some) => {
                CallForm { func: &self.func, input: &self.input, some }.mut_static_call(ctx, input)
            }
            None => CallEval { func: &self.func, input: &self.input }.mut_static_call(ctx, input),
        }
    }
}

impl TryFrom<PrimMode> for CallMode {
    type Error = ();

    fn try_from(mode: PrimMode) -> Result<Self, Self::Error> {
        let Some(code) = mode.call else {
            return Err(());
        };
        let some = match code {
            CodeMode::Form => Some(Map::default()),
            CodeMode::Eval => None,
        };
        Ok(Self { some, func: Some(mode.into()), input: Some(mode.into()) })
    }
}
