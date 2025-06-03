use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::Val;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::free_impl;
use crate::solver::SOLVER;

#[derive(Clone)]
pub(crate) struct SolvePrelude {
    pub(crate) get_solver: FreeStaticPrimFuncVal,
    pub(crate) set_solver: FreeStaticPrimFuncVal,
}

impl Default for SolvePrelude {
    fn default() -> Self {
        SolvePrelude { get_solver: get_solver(), set_solver: set_solver() }
    }
}

impl Prelude for SolvePrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.get_solver.put(ctx);
        self.set_solver.put(ctx);
    }
}

fn get_solver() -> FreeStaticPrimFuncVal {
    FreeFn { id: "solver!", f: free_impl(fn_get_solver), mode: FuncMode::default() }.free_static()
}

fn fn_get_solver(_input: Val) -> Val {
    SOLVER.with(|solver| {
        let Ok(solver) = solver.try_borrow() else {
            return Val::default();
        };
        match &*solver {
            Some(solver) => Val::Func(solver.clone()),
            _ => Val::default(),
        }
    })
}

fn set_solver() -> FreeStaticPrimFuncVal {
    FreeFn { id: "set_solver!", f: free_impl(fn_set_solver), mode: FuncMode::default() }
        .free_static()
}

fn fn_set_solver(input: Val) -> Val {
    let new_solver = match input {
        Val::Unit(_) => None,
        Val::Func(solver) => Some(solver),
        _ => return Val::default(),
    };
    SOLVER.with(|solver| {
        let Ok(mut solver) = solver.try_borrow_mut() else {
            return;
        };
        *solver = new_solver;
    });
    Val::default()
}
