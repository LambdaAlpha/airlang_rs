use std::cell::RefCell;

use crate::ConstCellFn;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeCellFn;
use crate::FreeStaticFn;
use crate::FuncVal;
use crate::MutCellFn;
use crate::MutStaticFn;
use crate::Val;

thread_local!(pub(crate) static SOLVER: RefCell<Option<FuncVal>> = RefCell::default());

pub(crate) struct Solver;

impl FreeStaticFn<Val, Val> for Solver {
    fn free_static_call(&self, question: Val) -> Val {
        let answer = SOLVER.with(|solver| {
            let mut solver = solver.try_borrow_mut().ok()?;
            let solver = solver.as_mut()?;
            let answer = if solver.is_cell() {
                solver.free_cell_call(question)
            } else {
                solver.free_static_call(question)
            };
            Some(answer)
        });
        if let Some(answer) = answer {
            if !answer.is_unit() {
                return answer;
            }
        }
        Val::default()
    }
}

impl ConstStaticFn<Ctx, Val, Val> for Solver {
    fn const_static_call(&self, mut ctx: ConstRef<Ctx>, question: Val) -> Val {
        let answer = SOLVER.with(|solver| {
            let mut solver = solver.try_borrow_mut().ok()?;
            let solver = solver.as_mut()?;
            let answer = if solver.is_cell() {
                solver.const_cell_call(ctx.reborrow(), question)
            } else {
                solver.const_static_call(ctx.reborrow(), question)
            };
            Some(answer)
        });
        if let Some(answer) = answer {
            if !answer.is_unit() {
                return answer;
            }
        }
        Val::default()
    }
}

impl MutStaticFn<Ctx, Val, Val> for Solver {
    fn mut_static_call(&self, ctx: &mut Ctx, question: Val) -> Val {
        let answer = SOLVER.with(|solver| {
            let mut solver = solver.try_borrow_mut().ok()?;
            let solver = solver.as_mut()?;
            let answer = if solver.is_cell() {
                solver.mut_cell_call(ctx, question)
            } else {
                solver.mut_static_call(ctx, question)
            };
            Some(answer)
        });
        if let Some(answer) = answer {
            if !answer.is_unit() {
                return answer;
            }
        }
        Val::default()
    }
}
