use airlang::prelude::DynPrimFn;
use airlang::prelude::Prelude;
use airlang::prelude::mut_impl;
use airlang::prelude::setup::default_dyn_mode;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::val::MutPrimFuncVal;
use airlang::semantics::val::Val;
use log::error;

use crate::prelude::BinPrelude;

pub struct EvalPrelude {
    pub reset: MutPrimFuncVal,
}

impl Default for EvalPrelude {
    fn default() -> Self {
        Self { reset: reset() }
    }
}

impl Prelude for EvalPrelude {
    fn put(self, ctx: &mut Ctx) {
        self.reset.put(ctx);
    }
}

// todo rename
pub fn reset() -> MutPrimFuncVal {
    DynPrimFn { id: "repl.reset", f: mut_impl(fn_reset), mode: default_dyn_mode() }.mut_()
}

fn fn_reset(ctx: &mut Val, _input: Val) -> Val {
    let Val::Ctx(ctx) = ctx else {
        error!("ctx {ctx:?} should be a ctx");
        return Val::default();
    };
    **ctx = BinPrelude::default().into();
    Val::default()
}
