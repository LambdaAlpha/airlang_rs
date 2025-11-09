use airlang::cfg::CfgMod;
use airlang::semantics::cfg::Cfg;
use airlang_ext::cfg2::StdCfg2;

use crate::cfg::BinCfg;

pub struct BinCfg2;

impl BinCfg2 {
    pub fn generate() -> Cfg {
        let mut cfg = Cfg::default();
        BinCfg::default().extend(&cfg);
        Self::extend(&mut cfg);
        cfg
    }

    pub fn extend(cfg: &mut Cfg) {
        StdCfg2::extend(cfg);
    }
}
