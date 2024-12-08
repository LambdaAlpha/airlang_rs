use crate::{
    FuncVal,
    Val,
    ctx::ref1::CtxMeta,
};

pub(crate) fn optimize<'a, Ctx>(_ctx: Ctx, _func: FuncVal, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    input
}
