use crate::cfg::adapter::CompAdapter;
use crate::cfg::adapter::PrimAdapter;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::FuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoreAdapter {
    Comp(CompAdapter),
    Func(FuncVal),
}

impl FreeFn<Cfg, Val, Val> for CoreAdapter {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        match self {
            CoreAdapter::Comp(comp) => comp.free_call(cfg, input),
            CoreAdapter::Func(func) => func.free_call(cfg, input),
        }
    }
}

impl ConstFn<Cfg, Val, Val, Val> for CoreAdapter {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        match self {
            CoreAdapter::Comp(comp) => comp.const_call(cfg, ctx, input),
            CoreAdapter::Func(func) => func.const_call(cfg, ctx, input),
        }
    }
}

impl MutFn<Cfg, Val, Val, Val> for CoreAdapter {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        match self {
            CoreAdapter::Comp(comp) => comp.mut_call(cfg, ctx, input),
            CoreAdapter::Func(func) => func.mut_call(cfg, ctx, input),
        }
    }
}

impl CoreAdapter {
    pub const fn id() -> Self {
        CoreAdapter::Comp(CompAdapter::id())
    }

    pub fn is_id(&self) -> bool {
        let CoreAdapter::Comp(adapter) = self else {
            return false;
        };
        adapter.is_id()
    }
}

impl From<PrimAdapter> for CoreAdapter {
    fn from(adapter: PrimAdapter) -> Self {
        CoreAdapter::Comp(CompAdapter::from(adapter))
    }
}
