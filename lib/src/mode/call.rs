use crate::CallVal;
use crate::CodeMode;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeStaticFn;
use crate::MutStaticFn;
use crate::PrimMode;
use crate::Val;
use crate::core::CallEval;
use crate::core::CallForm;
use crate::mode::Mode;
use crate::mode::ModeFn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallMode {
    pub code: CodeMode,
    pub func: Option<Mode>,
    pub input: Option<Mode>,
}

impl ModeFn for CallMode {}

impl FreeStaticFn<CallVal, Val> for CallMode {
    fn free_static_call(&self, input: CallVal) -> Val {
        match self.code {
            CodeMode::Form => {
                CallForm { func: &self.func, input: &self.input }.free_static_call(input)
            }
            CodeMode::Eval => {
                CallEval { func: &self.func, input: &self.input }.free_static_call(input)
            }
        }
    }
}

impl ConstStaticFn<Ctx, CallVal, Val> for CallMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: CallVal) -> Val {
        match self.code {
            CodeMode::Form => {
                CallForm { func: &self.func, input: &self.input }.const_static_call(ctx, input)
            }
            CodeMode::Eval => {
                CallEval { func: &self.func, input: &self.input }.const_static_call(ctx, input)
            }
        }
    }
}

impl MutStaticFn<Ctx, CallVal, Val> for CallMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: CallVal) -> Val {
        match self.code {
            CodeMode::Form => {
                CallForm { func: &self.func, input: &self.input }.mut_static_call(ctx, input)
            }
            CodeMode::Eval => {
                CallEval { func: &self.func, input: &self.input }.mut_static_call(ctx, input)
            }
        }
    }
}

impl TryFrom<PrimMode> for CallMode {
    type Error = ();

    fn try_from(mode: PrimMode) -> Result<Self, Self::Error> {
        let Some(code) = mode.call else {
            return Err(());
        };
        Ok(Self { code, func: Some(mode.into()), input: Some(mode.into()) })
    }
}
