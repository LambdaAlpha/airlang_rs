use crate::{
    FuncVal,
    Val,
    ctx::ref1::CtxMeta,
};

pub(crate) fn optimize<'a, Ctx>(_ctx: Ctx, _func: FuncVal, input: Val) -> Val
where Ctx: CtxMeta<'a> {
    input
}

pub(crate) fn solve<'a, Ctx>(_ctx: Ctx, _func: FuncVal, _output: Val) -> Val
where Ctx: CtxMeta<'a> {
    Val::default()
}
