use std::cell::RefCell;

use log::error;
use rustc_hash::FxHashMap;

use super::func::ConstCellFn;
use super::func::ConstStaticFn;
use super::func::FreeCellFn;
use super::func::FreeStaticFn;
use super::func::MutCellFn;
use super::func::MutStaticFn;
use super::val::FuncVal;
use super::val::Val;
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

impl FreeStaticFn<Val, Val> for Solve {
    fn free_static_call(&self, input: Val) -> Val {
        let answer = REVERSE_MAP.with(|map| {
            let Ok(mut map) = map.try_borrow_mut() else {
                error!("reverse map should be mutable");
                return None;
            };
            let reverse = map.get_mut(&self.func.id())?;
            let output = reverse.free_cell_call(input.clone());
            Some(output)
        });
        if let Some(answer) = answer {
            return answer;
        }
        let answer = SOLVER.with(|solver| {
            let mut solver = solver.try_borrow_mut().ok()?;
            let func_input = Val::Pair(Pair::new(Val::Func(self.func.clone()), input).into());
            let answer = solver.free_cell_call(func_input);
            Some(answer)
        });
        if let Some(answer) = answer {
            return answer;
        }
        Val::default()
    }
}

impl ConstStaticFn<Val, Val, Val> for Solve {
    fn const_static_call(&self, mut ctx: ConstRef<Val>, input: Val) -> Val {
        let answer = REVERSE_MAP.with(|map| {
            let Ok(mut map) = map.try_borrow_mut() else {
                error!("reverse map should be mutable");
                return None;
            };
            let reverse = map.get_mut(&self.func.id())?;
            let output = reverse.const_cell_call(ctx.reborrow(), input.clone());
            Some(output)
        });
        if let Some(answer) = answer {
            return answer;
        }
        let answer = SOLVER.with(|solver| {
            let mut solver = solver.try_borrow_mut().ok()?;
            let func_input = Val::Pair(Pair::new(Val::Func(self.func.clone()), input).into());
            let answer = solver.const_cell_call(ctx, func_input);
            Some(answer)
        });
        if let Some(answer) = answer {
            return answer;
        }
        Val::default()
    }
}

impl MutStaticFn<Val, Val, Val> for Solve {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        let answer = REVERSE_MAP.with(|map| {
            let Ok(mut map) = map.try_borrow_mut() else {
                error!("reverse map should be mutable");
                return None;
            };
            let reverse = map.get_mut(&self.func.id())?;
            let output = reverse.mut_cell_call(ctx, input.clone());
            Some(output)
        });
        if let Some(answer) = answer {
            return answer;
        }
        let answer = SOLVER.with(|solver| {
            let mut solver = solver.try_borrow_mut().ok()?;
            let func_input = Val::Pair(Pair::new(Val::Func(self.func.clone()), input).into());
            let answer = solver.mut_cell_call(ctx, func_input);
            Some(answer)
        });
        if let Some(answer) = answer {
            return answer;
        }
        Val::default()
    }
}
