use crate::cfg::CfgMod;
use crate::semantics::cfg::Cfg;

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
