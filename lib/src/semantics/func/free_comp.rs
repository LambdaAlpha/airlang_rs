use super::FreeFn;
use super::comp::FreeComposite;
use crate::semantics::cfg::Cfg;
use crate::semantics::memo::Memo;
use crate::semantics::val::Val;
use crate::type_::Key;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeCompFunc {
    pub(crate) id: Key,
    pub(crate) raw_input: bool,
    pub(crate) comp: FreeComposite,
    pub(crate) memo: Memo,
}

impl FreeFn<Cfg, Val, Val> for FreeCompFunc {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        self.comp.call(cfg, &mut self.memo.clone(), input)
    }
}
