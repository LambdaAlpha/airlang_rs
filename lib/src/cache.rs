use std::ops::Deref;

use crate::{
    FuncVal,
    Val,
    case::Case,
    ctx::ref1::CtxMeta,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cache<F, I, O>(Case<F, I, O>);

impl Cache<Val, Val, Val> {
    pub(crate) fn new<'a, Ctx>(ctx: Ctx, func: FuncVal, input: Val) -> Self
    where
        Ctx: CtxMeta<'a>,
    {
        let output = func.transform(ctx, input.clone());
        let func = Val::Func(func);
        let case = Case::new(func, input, output);
        Self(case)
    }
}

impl<F, I, O> Deref for Cache<F, I, O> {
    type Target = Case<F, I, O>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
