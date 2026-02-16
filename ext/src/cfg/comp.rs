use airlang::cfg::CfgMod;
use airlang::cfg::comp::BaseCompCfg;
use airlang::semantics::cfg::Cfg;

use crate::cfg::prim::ExtPrimCfg;

pub struct ExtCompCfg;

impl ExtCompCfg {
    pub fn generate() -> Cfg {
        let mut cfg = Cfg::default();
        ExtPrimCfg::default().extend(&mut cfg);
        Self::extend(&mut cfg);
        cfg
    }

    pub fn extend(cfg: &mut Cfg) {
        BaseCompCfg::extend(cfg);
    }
}
