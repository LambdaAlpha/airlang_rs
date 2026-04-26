use airlang::cfg::CfgMod;
use airlang::cfg::comp::BaseCompCfg;
use airlang::cfg::prelude;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::Val;
use airlang_ext::cfg::comp::ExtCompCfg;

use crate::cfg::prim::DevPrimCfg;

pub struct DevCompCfg;

impl DevCompCfg {
    pub fn generate() -> Cfg {
        let mut cfg = Cfg::default();
        DevPrimCfg::default().extend(&mut cfg);
        let mut ctx = prelude(&mut cfg);
        Self::extend(&mut cfg, &mut ctx);
        cfg
    }

    pub fn extend(cfg: &mut Cfg, ctx: &mut Val) {
        ExtCompCfg::extend(cfg, ctx);

        BaseCompCfg::run(cfg, ctx, include_str!("../air/first.air"), "/first");

        BaseCompCfg::run(cfg, ctx, include_str!("../air/last.air"), "/last");
    }
}
