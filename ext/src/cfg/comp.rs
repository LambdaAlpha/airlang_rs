use airlang::cfg::CfgMod;
use airlang::cfg::comp::BaseCompCfg;
use airlang::cfg::prelude;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::Val;

use crate::cfg::prim::ExtPrimCfg;

pub struct ExtCompCfg;

impl ExtCompCfg {
    pub fn generate() -> Cfg {
        let mut cfg = Cfg::default();
        ExtPrimCfg::default().extend(&mut cfg);
        let mut ctx = prelude(&mut cfg);
        Self::extend(&mut cfg, &mut ctx);
        cfg
    }

    pub fn extend(cfg: &mut Cfg, ctx: &mut Val) {
        BaseCompCfg::extend(cfg, ctx);

        BaseCompCfg::run_sequence(cfg, ctx, &[
            (include_str!("../air/first.air"), "/first"),
            (include_str!("../air/last.air"), "/last"),
        ]);
    }
}
