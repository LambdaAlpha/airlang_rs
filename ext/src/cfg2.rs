use airlang::cfg::CfgMod;
use airlang::cfg2::CoreCfg2;
use airlang::semantics::cfg::Cfg;

use crate::cfg::StdCfg;

pub struct StdCfg2;

impl StdCfg2 {
    pub fn generate() -> Cfg {
        let mut cfg = Cfg::default();
        StdCfg::default().extend(&mut cfg);
        Self::extend(&mut cfg);
        cfg
    }

    pub fn extend(cfg: &mut Cfg) {
        CoreCfg2::extend(cfg);
    }
}
