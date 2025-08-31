use super::Library;
use crate::cfg::CfgMod;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;

// todo design
#[derive(Clone)]
pub struct NumberLib {}

#[expect(clippy::derivable_impls)]
impl Default for NumberLib {
    fn default() -> Self {
        NumberLib {}
    }
}

impl CfgMod for NumberLib {
    fn extend(self, _cfg: &Cfg) {}
}

impl Library for NumberLib {
    fn prelude(&self, _ctx: &mut Ctx) {}
}
