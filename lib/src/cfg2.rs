use log::error;

use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::composite_call;
use crate::semantics::memo::Memo;
use crate::semantics::val::Val;
use crate::syntax::parse;

pub struct CoreCfg2;

impl CoreCfg2 {
    pub fn generate() -> Cfg {
        let mut cfg = Cfg::default();
        CoreCfg::default().extend(&cfg);
        Self::extend(&mut cfg);
        cfg
    }

    pub fn extend(cfg: &mut Cfg) {
        let mut ctx = CoreCfg::prelude(cfg).expect("prelude should be ready");
        let ctx = &mut ctx;

        Self::run(cfg, ctx, include_str!("air/first.air"), "/first").unwrap();

        // don't depend on the execution order {
        Self::run(cfg, ctx, include_str!("air/lib/unit.air"), "/lib/unit").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/bit.air"), "/lib/bit").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/symbol.air"), "/lib/symbol").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/text.air"), "/lib/text").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/integer.air"), "/lib/integer").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/number.air"), "/lib/number").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/byte.air"), "/lib/byte").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/pair.air"), "/lib/pair").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/call.air"), "/lib/call").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/list.air"), "/lib/list").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/map.air"), "lib/map").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/link.air"), "/lib/link").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/configuration.air"), "/lib/configuration")
            .unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/memory.air"), "/lib/memory").unwrap();
        Self::run(cfg, ctx, include_str!("air/lib/function.air"), "/lib/function").unwrap();
        // } don't depend on the execution order

        Self::run(cfg, ctx, include_str!("air/last.air"), "/last").unwrap();
    }

    pub fn run(cfg: &mut Cfg, ctx: &mut Memo, source: &str, path: &str) -> Option<Val> {
        let input = match parse(source) {
            Ok(input) => input,
            Err(err) => {
                error!("parse {path} failed: {err}");
                return None;
            }
        };
        let output = composite_call(cfg, ctx, input);
        Some(output)
    }
}
