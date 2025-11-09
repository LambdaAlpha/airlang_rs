use log::info;
use log::trace;

use crate::cfg::CoreCfg;
use crate::semantics::cfg::Cfg;
use crate::semantics::cfg::StepsExceed;
use crate::semantics::func::composite_call;
use crate::semantics::memo::Memo;
use crate::semantics::val::Val;

#[derive(Debug, Clone)]
pub struct Air {
    cfg: Cfg,
    memo: Memo,
}

impl Air {
    pub fn new(cfg: Cfg) -> Option<Self> {
        info!("cfg len {}", cfg.len());
        let memo = CoreCfg::prelude(&cfg).unwrap();
        Some(Self { cfg, memo })
    }

    pub fn interpret(&mut self, input: Val) -> Val {
        let old_steps = self.cfg.steps();
        let output = StepsExceed::catch(|| composite_call(&mut self.cfg, &mut self.memo, input));
        let new_steps = self.cfg.steps();
        trace!("takes {} steps, remains {} steps", old_steps - new_steps, new_steps);
        output.unwrap_or_default()
    }

    pub fn memo_mut(&mut self) -> &mut Memo {
        &mut self.memo
    }

    pub fn cfg_mut(&mut self) -> &mut Cfg {
        &mut self.cfg
    }
}
