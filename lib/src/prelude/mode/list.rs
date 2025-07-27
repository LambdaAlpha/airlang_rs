use super::Mode;
use super::PrimMode;
use crate::semantics::core::ListForm;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::func::SetupFn;
use crate::semantics::val::ListVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::List;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListMode {
    pub head: List<Option<Mode>>,
    pub tail: Option<Mode>,
}

impl SetupFn for ListMode {}

impl FreeStaticFn<ListVal, Val> for ListMode {
    fn free_static_call(&self, input: ListVal) -> Val {
        ListForm { head: &self.head, tail: &self.tail }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, ListVal, Val> for ListMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: ListVal) -> Val {
        ListForm { head: &self.head, tail: &self.tail }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, ListVal, Val> for ListMode {
    fn mut_static_call(&self, ctx: &mut Val, input: ListVal) -> Val {
        ListForm { head: &self.head, tail: &self.tail }.mut_static_call(ctx, input)
    }
}

impl From<PrimMode> for ListMode {
    fn from(mode: PrimMode) -> Self {
        Self { head: List::default(), tail: Some(mode.into()) }
    }
}
