use std::cell::RefCell;
use std::rc::Rc;

use super::func::ConstCellFn;
use super::func::ConstStaticFn;
use super::func::FreeCellFn;
use super::func::FreeStaticFn;
use super::func::FreeStaticPrimFunc;
use super::func::MutCellFn;
use super::func::MutStaticFn;
use super::func::Setup;
use super::func::default_setup;
use super::val::FuncVal;
use super::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;
use crate::type_::Unit;

thread_local!(pub(crate) static SOLVER: RefCell<FuncVal> = RefCell::new(unit_solver()));

pub(crate) fn set_solver(solver: FuncVal) {
    SOLVER.with(|s| {
        let Ok(mut s) = s.try_borrow_mut() else {
            return;
        };
        *s = solver;
    });
}

pub(crate) fn unit_solver() -> FuncVal {
    let default_setup = default_setup();
    FuncVal::FreeStaticPrim(
        FreeStaticPrimFunc {
            id: Symbol::from_str_unchecked("unit_solver"),
            fn_: Rc::new(UnitSolver),
            setup: Some(Setup { forward: default_setup.clone(), reverse: default_setup }),
        }
        .into(),
    )
}

struct UnitSolver;

impl FreeStaticFn<Val, Val> for UnitSolver {
    fn free_static_call(&self, _input: Val) -> Val {
        Val::Unit(Unit)
    }
}

// todo design knowledge base

// todo design default solve

pub(super) struct Solve;

impl FreeStaticFn<Val, Val> for Solve {
    fn free_static_call(&self, question: Val) -> Val {
        let answer = SOLVER.with(|solver| {
            let mut solver = solver.try_borrow_mut().ok()?;
            let answer = if solver.is_cell() {
                solver.free_cell_call(question)
            } else {
                solver.free_static_call(question)
            };
            Some(answer)
        });
        if let Some(answer) = answer
            && !answer.is_unit()
        {
            return answer;
        }
        Val::default()
    }
}

impl ConstStaticFn<Val, Val, Val> for Solve {
    fn const_static_call(&self, mut ctx: ConstRef<Val>, question: Val) -> Val {
        let answer = SOLVER.with(|solver| {
            let mut solver = solver.try_borrow_mut().ok()?;
            let answer = if solver.is_cell() {
                solver.const_cell_call(ctx.reborrow(), question)
            } else {
                solver.const_static_call(ctx.reborrow(), question)
            };
            Some(answer)
        });
        if let Some(answer) = answer
            && !answer.is_unit()
        {
            return answer;
        }
        Val::default()
    }
}

impl MutStaticFn<Val, Val, Val> for Solve {
    fn mut_static_call(&self, ctx: &mut Val, question: Val) -> Val {
        let answer = SOLVER.with(|solver| {
            let mut solver = solver.try_borrow_mut().ok()?;
            let answer = if solver.is_cell() {
                solver.mut_cell_call(ctx, question)
            } else {
                solver.mut_static_call(ctx, question)
            };
            Some(answer)
        });
        if let Some(answer) = answer
            && !answer.is_unit()
        {
            return answer;
        }
        Val::default()
    }
}
