use std::cell::RefCell;

use log::error;
use rustc_hash::FxHashMap;

use super::func::ConstFn;
use super::func::FreeFn;
use super::func::MutFn;
use super::val::FuncVal;
use super::val::Val;
use crate::semantics::cfg::Cfg;
use crate::type_::ConstRef;
use crate::type_::Pair;
use crate::type_::Symbol;

thread_local!(pub(crate) static SOLVER: RefCell<FuncVal> = RefCell::default());

// todo design knowledge base
thread_local!(pub(crate) static REVERSE_MAP: RefCell<FxHashMap<Symbol, FuncVal>> = RefCell::new(FxHashMap::default()));

pub(crate) fn set_solver(solver: FuncVal) {
    SOLVER.with(|s| {
        let Ok(mut s) = s.try_borrow_mut() else {
            error!("solver variable should be mutable");
            return;
        };
        *s = solver;
    });
}

// todo design default solve

pub(super) struct Solve {
    pub(super) func: FuncVal,
}

impl FreeFn<Cfg, Val, Val> for Solve {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        let answer = REVERSE_MAP.with(|map| {
            let reverse = map.borrow().get(&self.func.id())?.clone();
            let output = reverse.free_call(cfg, input.clone());
            Some(output)
        });
        if let Some(answer) = answer {
            return answer;
        }
        SOLVER.with(|solver| {
            let solver = solver.borrow().clone();
            let func_input = Val::Pair(Pair::new(Val::Func(self.func.clone()), input).into());
            solver.free_call(cfg, func_input)
        })
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Solve {
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Val>, input: Val) -> Val {
        let answer = REVERSE_MAP.with(|map| {
            let reverse = map.borrow().get(&self.func.id())?.clone();
            let output = reverse.const_call(cfg, ctx.reborrow(), input.clone());
            Some(output)
        });
        if let Some(answer) = answer {
            return answer;
        }
        SOLVER.with(|solver| {
            let solver = solver.borrow().clone();
            let func_input = Val::Pair(Pair::new(Val::Func(self.func.clone()), input).into());
            solver.const_call(cfg, ctx, func_input)
        })
    }
}

impl MutFn<Cfg, Val, Val, Val> for Solve {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        let answer = REVERSE_MAP.with(|map| {
            let reverse = map.borrow().get(&self.func.id())?.clone();
            let output = reverse.mut_call(cfg, ctx, input.clone());
            Some(output)
        });
        if let Some(answer) = answer {
            return answer;
        }
        SOLVER.with(|solver| {
            let solver = solver.borrow().clone();
            let func_input = Val::Pair(Pair::new(Val::Func(self.func.clone()), input).into());
            solver.mut_call(cfg, ctx, func_input)
        })
    }
}
