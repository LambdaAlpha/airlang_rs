use derive_more::Constructor;

use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::composite_call;
use crate::semantics::solve::set_solver;
use crate::semantics::val::FuncVal;
use crate::semantics::val::Val;

/// init thread local solver
pub fn init_solver(solver: FuncVal) {
    set_solver(solver);
}

#[derive(Debug, Clone, Constructor)]
pub struct Air {
    cfg: Cfg,
    ctx: Ctx,
}

impl Air {
    pub fn interpret(&mut self, input: Val) -> Val {
        composite_call(&mut self.cfg, &mut self.ctx, input)
    }

    pub fn ctx_mut(&mut self) -> &mut Ctx {
        &mut self.ctx
    }
}
