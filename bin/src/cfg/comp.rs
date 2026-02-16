use airlang::cfg::CfgMod;
use airlang::semantics::cfg::Cfg;
use airlang_ext::cfg::comp::ExtCompCfg;

use crate::cfg::prim::BinPrimCfg;

pub struct BinCompCfg;

impl BinCompCfg {
    pub fn generate() -> Cfg {
        let mut cfg = Cfg::default();
        BinPrimCfg::default().extend(&mut cfg);
        Self::extend(&mut cfg);
        cfg
    }

    pub fn extend(cfg: &mut Cfg) {
        ExtCompCfg::extend(cfg);
    }
}
