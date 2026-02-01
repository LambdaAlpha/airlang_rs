use crate::cfg::CoreCfg;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::func::CtxFn;
use crate::semantics::val::MapVal;
use crate::semantics::val::Val;

#[derive(Clone)]
pub struct Air {
    cfg: Cfg,
    ctx: Val,
}

impl Air {
    pub fn new(mut cfg: Cfg) -> Option<Self> {
        let ctx = CoreCfg::prelude(&mut cfg, "prelude")?;
        let ctx = Val::Map(MapVal::from(ctx));
        Some(Self { cfg, ctx })
    }

    pub fn interpret(&mut self, input: Val) -> Val {
        Eval.ctx_call(&mut self.cfg, &mut self.ctx, input)
    }

    pub fn ctx(&self) -> &Val {
        &self.ctx
    }

    pub fn ctx_mut(&mut self) -> &mut Val {
        &mut self.ctx
    }

    pub fn cfg(&self) -> &Cfg {
        &self.cfg
    }

    pub fn cfg_mut(&mut self) -> &mut Cfg {
        &mut self.cfg
    }
}
