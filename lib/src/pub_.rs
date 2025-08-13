pub use crate::prelude::initial_ctx;

_____!();

use derive_more::Constructor;

use crate::prelude::Prelude;
use crate::prelude::mode::FuncMode;
use crate::prelude::mode::Mode;
use crate::prelude::set_prelude;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::composite_call;
use crate::semantics::solve::set_solver;
use crate::semantics::val::FuncVal;
use crate::semantics::val::Val;

/// init thread local prelude
/// this method should be called before instantiating [`Air`] or calling [`initial_ctx`]
pub fn init_prelude(prelude: Box<dyn Prelude>) {
    set_prelude(prelude);
}

/// init thread local solver
pub fn init_solver(solver: FuncVal) {
    set_solver(solver);
}

#[derive(Debug, Clone, Constructor)]
pub struct Air {
    mode: Mode,
    ctx: Ctx,
}

impl Air {
    pub fn interpret(&mut self, input: Val) -> Val {
        composite_call(&self.mode, &mut self.ctx, input)
    }

    pub fn ctx_mut(&mut self) -> &mut Ctx {
        &mut self.ctx
    }
}

impl Default for Air {
    fn default() -> Self {
        Self { mode: FuncMode::default_mode(), ctx: initial_ctx() }
    }
}
