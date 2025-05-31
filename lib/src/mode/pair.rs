use crate::ConstRef;
use crate::ConstStaticFn;
use crate::FreeStaticFn;
use crate::MutStaticFn;
use crate::PairVal;
use crate::PrimMode;
use crate::Val;
use crate::core::PairForm;
use crate::mode::Mode;
use crate::mode::ModeFn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PairMode {
    pub first: Option<Mode>,
    pub second: Option<Mode>,
}

impl ModeFn for PairMode {}

impl FreeStaticFn<PairVal, Val> for PairMode {
    fn free_static_call(&self, input: PairVal) -> Val {
        PairForm { first: &self.first, second: &self.second }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, PairVal, Val> for PairMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: PairVal) -> Val {
        PairForm { first: &self.first, second: &self.second }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, PairVal, Val> for PairMode {
    fn mut_static_call(&self, ctx: &mut Val, input: PairVal) -> Val {
        PairForm { first: &self.first, second: &self.second }.mut_static_call(ctx, input)
    }
}

impl From<PrimMode> for PairMode {
    fn from(mode: PrimMode) -> Self {
        Self { first: Some(mode.into()), second: Some(mode.into()) }
    }
}
