use airlang::cfg::CfgMod;
use airlang::cfg::comp::BaseCompCfg;
use airlang::cfg::prelude;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::Val;
use airlang::type_::Map;
use airlang_ext::cfg::comp::ExtCompCfg;

use crate::cfg::prim::BinPrimCfg;

pub struct BinCompCfg;

impl BinCompCfg {
    pub fn generate() -> Cfg {
        let mut cfg = Map::default();
        BinPrimCfg::default().export(&mut cfg);
        let mut ctx = prelude(&cfg);
        let mut cfg = Cfg::new(cfg);
        Self::export(&mut cfg, &mut ctx);
        cfg
    }

    pub fn export(cfg: &mut Cfg, ctx: &mut Val) {
        ExtCompCfg::export(cfg, ctx);

        BaseCompCfg::run_sequence(cfg, ctx, &[
            (include_str!("../air/first.air"), "/first"),
            (include_str!("../air/lib/repl.air"), "/lib/repl"),
            (include_str!("../air/last.air"), "/last"),
        ]);
    }
}
