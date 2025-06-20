use crate::prelude::Prelude;
use crate::prelude::initial_ctx;
use crate::prelude::mode::FuncMode;
use crate::prelude::mode::Mode;
use crate::prelude::set_prelude;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::composite_call;
use crate::semantics::solver::set_solver;
use crate::semantics::val::FuncVal;
use crate::semantics::val::Val;

#[derive(Debug, Clone)]
pub struct Air {
    mode: Option<Mode>,
    ctx: Ctx,
}

impl Air {
    /// init thread local prelude
    /// this method should be called before instantiating `Air` or calling `initial_ctx`
    pub fn init_prelude(prelude: Box<dyn Prelude>) {
        set_prelude(prelude);
    }

    /// init thread local solver
    /// this method should be called before calling `interpret`
    pub fn init_solver(solver: FuncVal) {
        set_solver(solver);
    }

    pub fn new(mode: Option<Mode>, ctx: Ctx) -> Self {
        Self { mode, ctx }
    }

    pub fn initial_ctx() -> Ctx {
        initial_ctx()
    }

    pub fn interpret(&mut self, input: Val) -> Val {
        composite_call(&self.mode, &mut self.ctx, input)
    }

    pub fn ctx_mut(&mut self) -> &mut Ctx {
        &mut self.ctx
    }
}

impl Default for Air {
    fn default() -> Self {
        Self { mode: FuncMode::default_mode(), ctx: Self::initial_ctx() }
    }
}
