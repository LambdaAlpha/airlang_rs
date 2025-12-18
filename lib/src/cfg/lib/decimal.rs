use crate::cfg::CfgMod;
use crate::semantics::cfg::Cfg;

// todo design
#[derive(Clone)]
pub struct DecimalLib {}

#[expect(clippy::derivable_impls)]
impl Default for DecimalLib {
    fn default() -> Self {
        DecimalLib {}
    }
}

impl CfgMod for DecimalLib {
    fn extend(self, _cfg: &Cfg) {}
}
