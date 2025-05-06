use crate::Call;
use crate::FuncMode;
use crate::FuncVal;
use crate::MutFnCtx;
use crate::Pair;
use crate::Val;
use crate::core::EvalCore;
use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::named_mut_fn;
use crate::utils::val::symbol;

#[derive(Clone)]
pub(crate) struct SolvePrelude {
    pub(crate) inverse: Named<FuncVal>,
    pub(crate) imply: Named<FuncVal>,
}

impl Default for SolvePrelude {
    fn default() -> Self {
        SolvePrelude { inverse: inverse(), imply: imply() }
    }
}

impl Prelude for SolvePrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.inverse.put(ctx);
        self.imply.put(ctx);
    }
}

const INVERSE: &str = "inverse!";

fn inverse() -> Named<FuncVal> {
    let id = INVERSE;
    let f = fn_inverse;
    let mode = FuncMode::default();
    named_mut_fn(id, f, mode)
}

// (f inverse! v) evaluates to . or any i that (f ; i) always evaluates to v
fn fn_inverse(mut ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let pair_clone = pair.clone();
    let Val::Func(func) = pair.first else {
        return Val::default();
    };
    let question = Val::Call(Call::new(symbol(INVERSE), Val::Pair(pair_clone.into())).into());
    if let Some(answer) = EvalCore::call_solver(ctx.reborrow(), question) {
        if !answer.is_unit() {
            return answer;
        }
    }
    crate::solver::inverse(ctx, func, pair.second)
}

const IMPLY: &str = "imply!";

fn imply() -> Named<FuncVal> {
    let id = IMPLY;
    let f = fn_imply;
    let mode = FuncMode::default();
    named_mut_fn(id, f, mode)
}

// (x imply! y) evaluates to . or true if x is a subset of y or false if x is not subset of y
fn fn_imply(mut ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let pair_clone = pair.clone();
    let question = Val::Call(Call::new(symbol(IMPLY), Val::Pair(pair_clone.into())).into());
    if let Some(answer) = EvalCore::call_solver(ctx.reborrow(), question) {
        if !answer.is_unit() {
            return answer;
        }
    }
    crate::solver::imply(ctx, pair.first, pair.second)
}
