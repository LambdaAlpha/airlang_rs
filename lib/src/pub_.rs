use log::info;
use log::trace;

use crate::cfg::CoreCfg;
use crate::semantics::cfg::Cfg;
use crate::semantics::cfg::StepsExceed;
use crate::semantics::core::Eval;
use crate::semantics::func::MutFn;
use crate::semantics::val::MapVal;
use crate::semantics::val::Val;

#[derive(Debug, Clone)]
pub struct Air {
    cfg: Cfg,
    ctx: Val,
}

impl Air {
    pub fn new(cfg: Cfg) -> Option<Self> {
        info!("cfg len {}", cfg.len());
        let ctx = CoreCfg::prelude(&cfg).unwrap();
        let ctx = Val::Map(MapVal::from(ctx));
        Some(Self { cfg, ctx })
    }

    pub fn interpret(&mut self, input: Val) -> Val {
        let old_steps = self.cfg.steps();
        let output = StepsExceed::catch(|| Eval.mut_call(&mut self.cfg, &mut self.ctx, input));
        let new_steps = self.cfg.steps();
        trace!("takes {} steps", old_steps - new_steps);
        output.unwrap_or_default()
    }

    pub fn ctx_mut(&mut self) -> &mut Val {
        &mut self.ctx
    }

    pub fn cfg_mut(&mut self) -> &mut Cfg {
        &mut self.cfg
    }
}
