use log::error;

use super::FreeFn;
use super::Prelude;
use super::PreludeCtx;
use super::free_impl;
use crate::prelude::mode::FuncMode;
use crate::prelude::mode::SymbolMode;
use crate::prelude::setup::default_free_mode;
use crate::prelude::setup::free_mode;
use crate::semantics::solver::REVERSE_MAP;
use crate::semantics::solver::SOLVER;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Map;
use crate::type_::Pair;

#[derive(Clone)]
pub struct SolvePrelude {
    pub solver: FreeStaticPrimFuncVal,
    pub set_solver: FreeStaticPrimFuncVal,
    pub reverse: FreeStaticPrimFuncVal,
    pub set_reverse: FreeStaticPrimFuncVal,
}

impl Default for SolvePrelude {
    fn default() -> Self {
        SolvePrelude {
            solver: solver(),
            set_solver: set_solver(),
            reverse: reverse(),
            set_reverse: set_reverse(),
        }
    }
}

impl Prelude for SolvePrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.solver.put(ctx);
        self.set_solver.put(ctx);
        self.reverse.put(ctx);
        self.set_reverse.put(ctx);
    }
}

pub fn solver() -> FreeStaticPrimFuncVal {
    FreeFn { id: "solver!", f: free_impl(fn_solver), mode: default_free_mode() }.free_static()
}

fn fn_solver(_input: Val) -> Val {
    SOLVER.with(|solver| {
        let Ok(solver) = solver.try_borrow() else {
            return Val::default();
        };
        Val::Func(solver.clone())
    })
}

pub fn set_solver() -> FreeStaticPrimFuncVal {
    FreeFn { id: "set_solver!", f: free_impl(fn_set_solver), mode: default_free_mode() }
        .free_static()
}

fn fn_set_solver(input: Val) -> Val {
    let Val::Func(new_solver) = input else {
        error!("input {input:?} should be a function");
        return Val::default();
    };
    SOLVER.with(|solver| {
        let Ok(mut solver) = solver.try_borrow_mut() else {
            error!("should not call this function inside a solver");
            return;
        };
        *solver = new_solver;
    });
    Val::default()
}

pub fn reverse() -> FreeStaticPrimFuncVal {
    FreeFn {
        id: "reverse!",
        f: free_impl(fn_reverse),
        mode: free_mode(FuncMode::symbol_mode(SymbolMode::Literal)),
    }
    .free_static()
}

fn fn_reverse(input: Val) -> Val {
    let Val::Symbol(id) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    REVERSE_MAP.with(|map| {
        let Ok(map) = map.try_borrow() else {
            error!("reverse map should be readable");
            return Val::default();
        };
        let Some(func) = map.get(&id) else {
            error!("reverse func of {id:?} should exist");
            return Val::default();
        };
        Val::Func(func.clone())
    })
}

pub fn set_reverse() -> FreeStaticPrimFuncVal {
    FreeFn {
        id: "set_reverse!",
        f: free_impl(fn_set_reverse),
        mode: free_mode(FuncMode::pair_mode(
            Map::default(),
            FuncMode::symbol_mode(SymbolMode::Literal),
            FuncMode::default_mode(),
        )),
    }
    .free_static()
}

fn fn_set_reverse(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(id) = pair.first else {
        error!("input.first {:?} should be a symbol", pair.first);
        return Val::default();
    };
    let Val::Func(reverse) = pair.second else {
        error!("input.second {:?} should be a function", pair.second);
        return Val::default();
    };
    REVERSE_MAP.with(|map| {
        let Ok(mut map) = map.try_borrow_mut() else {
            error!("reverse map should be mutable");
            return;
        };
        map.insert(id, reverse);
    });
    Val::default()
}
