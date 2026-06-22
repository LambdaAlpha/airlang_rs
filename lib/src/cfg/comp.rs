use crate::cfg::CfgMod;
use crate::cfg::error::ABORT_MSG;
use crate::cfg::error::ABORT_TYPE;
use crate::cfg::import;
use crate::cfg::prelude;
use crate::cfg::prim::BasePrimCfg;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::DynFunc;
use crate::semantics::val::Val;
use crate::type_::Map;

pub struct BaseCompCfg;

impl BaseCompCfg {
    pub fn generate() -> Cfg {
        let mut cfg = Map::default();
        BasePrimCfg::default().export(&mut cfg);
        let mut ctx = prelude(&cfg);
        let mut cfg = Cfg::new(cfg);
        Self::export(&mut cfg, &mut ctx);
        cfg
    }

    pub fn export(cfg: &mut Cfg, ctx: &mut Val) {
        Self::run_sequence(cfg, ctx, &[
            (include_str!("../air/first.air"), "/first"),
            // don't depend on the execution order {
            (include_str!("../air/lib/unit.air"), "/lib/unit"),
            (include_str!("../air/lib/bit.air"), "/lib/bit"),
            (include_str!("../air/lib/key.air"), "/lib/key"),
            (include_str!("../air/lib/text.air"), "/lib/text"),
            (include_str!("../air/lib/integer.air"), "/lib/integer"),
            (include_str!("../air/lib/decimal.air"), "/lib/decimal"),
            (include_str!("../air/lib/byte.air"), "/lib/byte"),
            (include_str!("../air/lib/cell.air"), "/lib/cell"),
            (include_str!("../air/lib/pair.air"), "/lib/pair"),
            (include_str!("../air/lib/list.air"), "/lib/list"),
            (include_str!("../air/lib/map.air"), "lib/map"),
            (include_str!("../air/lib/quote.air"), "/lib/quote"),
            (include_str!("../air/lib/call.air"), "/lib/call"),
            (include_str!("../air/lib/solve.air"), "/lib/solve"),
            (include_str!("../air/lib/fact.air"), "/lib/fact"),
            (include_str!("../air/lib/link.air"), "/lib/link"),
            (include_str!("../air/lib/config.air"), "/lib/config"),
            (include_str!("../air/lib/function.air"), "/lib/function"),
            (include_str!("../air/lib/value.air"), "/lib/value"),
            (include_str!("../air/lib/error.air"), "/lib/error"),
            (include_str!("../air/lib/language.air"), "/lib/language"),
            // } don't depend on the execution order
            (include_str!("../air/last.air"), "/last"),
        ]);
    }

    pub fn run_sequence(cfg: &mut Cfg, ctx: &mut Val, list: &[(&str, &str)]) {
        for (source, path) in list {
            Self::run(cfg, ctx, source, path);
        }
    }

    pub fn run(cfg: &mut Cfg, ctx: &mut Val, source: &str, path: &str) -> Val {
        let input: Val = match source.parse() {
            Ok(input) => input,
            Err(err) => panic!("stage 2: failed to parse {path}: {err}"),
        };
        let output = Eval.call(cfg, Ctx::new_mut(ctx), input);
        if cfg.aborted {
            let type_ = import(&cfg.map, ABORT_TYPE);
            let msg = import(&cfg.map, ABORT_MSG);
            match (type_, msg) {
                (Some(type_), Some(msg)) => panic!("stage 2: aborted by {type_}: {msg}"),
                (None, Some(msg)) => panic!("stage 2: aborted: {msg}"),
                (Some(type_), None) => panic!("stage 2: aborted by {type_}"),
                (None, None) => panic!("stage 2: aborted"),
            }
        }
        output
    }
}
