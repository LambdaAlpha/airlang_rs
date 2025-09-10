use super::ConstFn;
use super::FreeFn;
use super::MutFn;
use super::comp::DynComposite;
use crate::semantics::cfg::Cfg;
use crate::semantics::memo::Memo;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::Symbol;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MutCompFunc {
    pub(crate) id: Symbol,
    pub(crate) comp: DynComposite,
    pub(crate) memo: Memo,
}

impl FreeFn<Cfg, Val, Val> for MutCompFunc {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        self.comp.free.call(cfg, &mut self.memo.clone(), input)
    }
}

impl ConstFn<Cfg, Val, Val, Val> for MutCompFunc {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        self.comp.call(cfg, &mut self.memo.clone(), ctx.into_dyn(), input)
    }
}

impl MutFn<Cfg, Val, Val, Val> for MutCompFunc {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        self.comp.call(cfg, &mut self.memo.clone(), DynRef::new_mut(ctx), input)
    }
}
