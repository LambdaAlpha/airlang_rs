use std::ops::Deref;

use crate::{
    ctx::CtxRef,
    ctx_access::CtxAccessor,
    transformer::{
        output::OutputBuilder,
        Transformer,
        ValBuilder,
    },
    val::func::FuncVal,
    AssertVal,
    Symbol,
    Val,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Answer {
    #[default]
    Unsolved,
    Unsolvable,
    Unverified(Val),
    Verified(Verified),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Verified(pub(crate) AssertVal);

pub(crate) fn solve<'a, Ctx>(mut ctx: Ctx, func: FuncVal, output: Val) -> Val
where
    Ctx: CtxAccessor<'a>,
{
    if !func.is_ctx_free() {
        return Val::default();
    }
    let Ok(meta) = ctx.reborrow().get_meta() else {
        return Val::default();
    };
    let Ok(solver) = meta.get_ref(Symbol::from_str(SOLVER)) else {
        return Val::default();
    };
    let Val::Func(FuncVal(solver)) = solver.clone() else {
        return Val::default();
    };
    let ask = ValBuilder.from_ask(Val::Func(func.clone()), output.clone());
    let input = solver.transform(ctx, ask);
    let Val::Answer(answer) = &input else {
        return Val::default();
    };
    let Answer::Verified(assert) = &**answer else {
        return input;
    };
    if *assert.func() != func || *assert.output() != output || !assert.is_verified() {
        return Val::default();
    }
    input
}

pub(crate) const SOLVER: &str = "solver";

impl Verified {
    pub fn new(assert_val: AssertVal) -> Option<Verified> {
        if assert_val.is_verified() {
            Some(Verified(assert_val))
        } else {
            None
        }
    }
}

impl Deref for Verified {
    type Target = AssertVal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
