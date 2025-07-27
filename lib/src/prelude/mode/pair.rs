use super::Mode;
use super::PrimMode;
use crate::semantics::core::PairForm;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::func::SetupFn;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PairMode {
    pub some: Map<Val, Option<Mode>>,
    pub first: Option<Mode>,
    pub second: Option<Mode>,
}

impl SetupFn for PairMode {}

impl FreeStaticFn<PairVal, Val> for PairMode {
    fn free_static_call(&self, input: PairVal) -> Val {
        PairForm { some: &self.some, first: &self.first, second: &self.second }
            .free_static_call(input)
    }
}

impl ConstStaticFn<Val, PairVal, Val> for PairMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: PairVal) -> Val {
        PairForm { some: &self.some, first: &self.first, second: &self.second }
            .const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, PairVal, Val> for PairMode {
    fn mut_static_call(&self, ctx: &mut Val, input: PairVal) -> Val {
        PairForm { some: &self.some, first: &self.first, second: &self.second }
            .mut_static_call(ctx, input)
    }
}

impl From<PrimMode> for PairMode {
    fn from(mode: PrimMode) -> Self {
        Self { some: Map::default(), first: Some(mode.into()), second: Some(mode.into()) }
    }
}
