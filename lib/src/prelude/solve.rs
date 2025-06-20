use super::FreeFn;
use super::FuncMode;
use super::Prelude;
use super::PreludeCtx;
use super::free_impl;
use crate::semantics::solver::SOLVER;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::Val;

#[derive(Clone)]
pub struct SolvePrelude {
    pub get_solver: FreeStaticPrimFuncVal,
    pub set_solver: FreeStaticPrimFuncVal,
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

pub fn get_solver() -> FreeStaticPrimFuncVal {
    FreeFn { id: "solver!", f: free_impl(fn_get_solver), mode: FuncMode::default() }.free_static()
}

fn fn_get_solver(_input: Val) -> Val {
    SOLVER.with(|solver| {
        let Ok(solver) = solver.try_borrow() else {
            return Val::default();
        };
        Val::Func(solver.clone())
    })
}

pub fn set_solver() -> FreeStaticPrimFuncVal {
    FreeFn { id: "set_solver!", f: free_impl(fn_set_solver), mode: FuncMode::default() }
        .free_static()
}

fn fn_set_solver(input: Val) -> Val {
    let Val::Func(new_solver) = input else {
        return Val::default();
    };
    SOLVER.with(|solver| {
        let Ok(mut solver) = solver.try_borrow_mut() else {
            return;
        };
        *solver = new_solver;
    });
    Val::default()
}
