use log::error;
use log::info;

use crate::cfg::CoreCfg;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::composite_call;
use crate::semantics::memo::Memo;
use crate::semantics::val::Val;
use crate::type_::Symbol;

#[derive(Debug, Clone)]
pub struct Air {
    cfg: Cfg,
    memo: Memo,
}

impl Air {
    pub fn new(cfg: Cfg) -> Option<Self> {
        info!("cfg len {}", cfg.len());
        let prelude = cfg.import(Symbol::from_str_unchecked(CoreCfg::PRELUDE));
        let Some(prelude) = prelude else {
            error!("prelude should exist in cfg");
            return None;
        };
        let Val::Memo(prelude) = prelude else {
            error!("prelude in cfg should be a memo");
            return None;
        };
        let memo = Memo::from(prelude);
        Some(Self { cfg, memo })
    }

    pub fn interpret(&mut self, input: Val) -> Val {
        composite_call(&mut self.cfg, &mut self.memo, input)
    }

    pub fn memo_mut(&mut self) -> &mut Memo {
        &mut self.memo
    }

    pub fn cfg_mut(&mut self) -> &mut Cfg {
        &mut self.cfg
    }
}
