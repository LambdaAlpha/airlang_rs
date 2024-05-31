use std::ops::Deref;

use crate::{
    ctx::ref1::{
        CtxMeta,
        CtxRef,
    },
    transformer::Transformer,
    val::func::FuncVal,
    Ask,
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
pub struct Verified(AssertVal);

pub(crate) fn solve<'a, Ctx>(mut ctx: Ctx, func: FuncVal, output: Val) -> Val
where
    Ctx: CtxMeta<'a>,
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
    let Val::Func(solver) = solver.clone() else {
        return Val::default();
    };
    let ask = Ask::new(Val::Func(func.clone()), output.clone());
    let ask = Val::Ask(ask.into());
    let input = solver.transform(ctx, ask);
    let Val::Answer(answer) = &input else {
        return Val::default();
    };
    let Answer::Verified(assert) = &**answer else {
        return input;
    };
    let Val::Func(assert_func) = assert.func() else {
        return Val::default();
    };
    if *assert_func != func || *assert.output() != output || !assert.is_verified() {
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

    pub fn unwrap(self) -> AssertVal {
        self.0
    }
}

impl Deref for Verified {
    type Target = AssertVal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
