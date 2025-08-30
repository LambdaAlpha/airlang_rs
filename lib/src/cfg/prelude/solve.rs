use log::error;

use super::FreePrimFn;
use super::Prelude;
use super::free_impl;
use crate::cfg::prelude::setup::default_free_mode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::solve::REVERSE_MAP;
use crate::semantics::solve::SOLVER;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Pair;

#[derive(Clone)]
pub struct SolvePrelude {
    pub solver: FreePrimFuncVal,
    pub set_solver: FreePrimFuncVal,
    pub reverse: FreePrimFuncVal,
    pub set_reverse: FreePrimFuncVal,
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
    fn put(self, ctx: &mut Ctx) {
        self.solver.put(ctx);
        self.set_solver.put(ctx);
        self.reverse.put(ctx);
        self.set_reverse.put(ctx);
    }
}

pub fn solver() -> FreePrimFuncVal {
    FreePrimFn { id: "solver!", f: free_impl(fn_solver), mode: default_free_mode() }.free()
}

fn fn_solver(_cfg: &mut Cfg, _input: Val) -> Val {
    SOLVER.with(|solver| Val::Func(solver.borrow().clone()))
}

pub fn set_solver() -> FreePrimFuncVal {
    FreePrimFn { id: "set_solver!", f: free_impl(fn_set_solver), mode: default_free_mode() }.free()
}

fn fn_set_solver(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(new_solver) = input else {
        error!("input {input:?} should be a function");
        return Val::default();
    };
    SOLVER.with(|solver| {
        *solver.borrow_mut() = new_solver;
    });
    Val::default()
}

pub fn reverse() -> FreePrimFuncVal {
    FreePrimFn { id: "reverse!", f: free_impl(fn_reverse), mode: default_free_mode() }.free()
}

fn fn_reverse(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Symbol(id) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    REVERSE_MAP.with(|map| {
        let map = map.borrow();
        let Some(func) = map.get(&id) else {
            error!("reverse func of {id:?} should exist");
            return Val::default();
        };
        Val::Func(func.clone())
    })
}

pub fn set_reverse() -> FreePrimFuncVal {
    FreePrimFn { id: "set_reverse!", f: free_impl(fn_set_reverse), mode: default_free_mode() }
        .free()
}

fn fn_set_reverse(_cfg: &mut Cfg, input: Val) -> Val {
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
        map.borrow_mut().insert(id, reverse);
    });
    Val::default()
}
