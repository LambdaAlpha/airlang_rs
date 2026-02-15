use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::func::DynFunc;
use crate::semantics::val::Val;
use crate::syntax::parse;
use crate::type_::Key;

pub struct CoreCfg2;

impl CoreCfg2 {
    pub fn generate() -> Cfg {
        let mut cfg = Cfg::default();
        CoreCfg::default().extend(&mut cfg);
        Self::extend(&mut cfg);
        cfg
    }

    pub fn extend(cfg: &mut Cfg) {
        let mut ctx = CoreCfg::prelude(cfg, "stage 2").expect("prelude should be ready");
        let ctx = &mut ctx;

        Self::run(cfg, ctx, include_str!("air/first.air"), "/first");

        // don't depend on the execution order {
        Self::run(cfg, ctx, include_str!("air/lib/unit.air"), "/lib/unit");
        Self::run(cfg, ctx, include_str!("air/lib/bit.air"), "/lib/bit");
        Self::run(cfg, ctx, include_str!("air/lib/key.air"), "/lib/key");
        Self::run(cfg, ctx, include_str!("air/lib/text.air"), "/lib/text");
        Self::run(cfg, ctx, include_str!("air/lib/integer.air"), "/lib/integer");
        Self::run(cfg, ctx, include_str!("air/lib/decimal.air"), "/lib/decimal");
        Self::run(cfg, ctx, include_str!("air/lib/byte.air"), "/lib/byte");
        Self::run(cfg, ctx, include_str!("air/lib/cell.air"), "/lib/cell");
        Self::run(cfg, ctx, include_str!("air/lib/pair.air"), "/lib/pair");
        Self::run(cfg, ctx, include_str!("air/lib/call.air"), "/lib/call");
        Self::run(cfg, ctx, include_str!("air/lib/list.air"), "/lib/list");
        Self::run(cfg, ctx, include_str!("air/lib/map.air"), "lib/map");
        Self::run(cfg, ctx, include_str!("air/lib/link.air"), "/lib/link");
        Self::run(cfg, ctx, include_str!("air/lib/config.air"), "/lib/config");
        Self::run(cfg, ctx, include_str!("air/lib/function.air"), "/lib/function");

        Self::run(cfg, ctx, include_str!("air/lib/language.air"), "/lib/language");
        // } don't depend on the execution order

        Self::run(cfg, ctx, include_str!("air/last.air"), "/last");
    }

    pub fn run(cfg: &mut Cfg, ctx: &mut Val, source: &str, path: &str) -> Val {
        let input: Val = match parse(source) {
            Ok(input) => input,
            Err(_) => panic!("stage 2: failed to parse {path}"),
        };
        let output = Eval.call(cfg, ctx, input);
        if cfg.is_aborted() {
            let type_ = cfg.import(Key::from_str_unchecked(Cfg::ABORT_TYPE));
            let msg = cfg.import(Key::from_str_unchecked(Cfg::ABORT_MSG));
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
