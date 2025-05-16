use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeStaticFn;
use crate::List;
use crate::ListVal;
use crate::MutStaticFn;
use crate::UniMode;
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

impl ConstStaticFn<Ctx, ListVal, Val> for ListMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: ListVal) -> Val {
        ListForm { head: &self.head, tail: &self.tail }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Ctx, ListVal, Val> for ListMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: ListVal) -> Val {
        ListForm { head: &self.head, tail: &self.tail }.mut_static_call(ctx, input)
    }
}

impl From<UniMode> for ListMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        ListMode { head: List::default(), tail: m }
    }
}
