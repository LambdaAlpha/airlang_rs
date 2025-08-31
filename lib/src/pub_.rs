use crate::cfg::CoreCfg;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::composite_call;
use crate::semantics::val::Val;
use crate::type_::Symbol;

#[derive(Debug, Clone)]
pub struct Air {
    cfg: Cfg,
    ctx: Ctx,
}

impl Air {
    pub fn new(cfg: Cfg) -> Self {
        let prelude = cfg.import(Symbol::from_str_unchecked(CoreCfg::PRELUDE));
        let prelude = prelude.expect("prelude should exist in cfg");
        let Val::Ctx(prelude) = prelude else {
            panic!("prelude in cfg should be a ctx");
        };
        let ctx = Ctx::from(prelude);
        Self { cfg, ctx }
    }

    pub fn interpret(&mut self, input: Val) -> Val {
        composite_call(&mut self.cfg, &mut self.ctx, input)
    }

    pub fn ctx_mut(&mut self) -> &mut Ctx {
        &mut self.ctx
    }

    pub fn cfg_mut(&mut self) -> &mut Cfg {
        &mut self.cfg
    }
}
