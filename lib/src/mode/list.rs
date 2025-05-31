use crate::ConstRef;
use crate::ConstStaticFn;
use crate::FreeStaticFn;
use crate::List;
use crate::ListVal;
use crate::MutStaticFn;
use crate::PrimMode;
use crate::Val;
use crate::core::ListForm;
use crate::mode::Mode;
use crate::mode::ModeFn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListMode {
    pub head: List<Option<Mode>>,
    pub tail: Option<Mode>,
}

impl ModeFn for ListMode {}

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
