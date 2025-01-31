use crate::{
    ConstFnCtx,
    Mode,
    MutFnCtx,
    Symbol,
    Val,
    ctx::{
        map::Invariant,
        ref1::CtxMeta,
    },
    func::comp::Composite,
};

pub(crate) fn const_func_transform<'a, Ctx>(
    inner: &mut crate::Ctx,
    ctx_name: Symbol,
    outer: Ctx,
    input_name: Symbol,
    input: Val,
    body_mode: &Mode,
    body: Val,
) -> Val
where
    Ctx: CtxMeta<'a>,
{
    Composite::put_input(inner, input_name, input);

    match outer.for_const_fn() {
        ConstFnCtx::Free(_ctx) => Composite::transform(body_mode, inner, body),
        ConstFnCtx::Const(ctx) => {
            let eval = |inner: &mut crate::Ctx| Composite::transform(body_mode, inner, body);
            Composite::with_ctx(inner, ctx.unwrap(), ctx_name, Invariant::Const, eval)
        }
    }
}

pub(crate) fn mut_func_transform<'a, Ctx>(
    inner: &mut crate::Ctx,
    ctx_name: Symbol,
    outer: Ctx,
    input_name: Symbol,
    input: Val,
    body_mode: &Mode,
    body: Val,
) -> Val
where
    Ctx: CtxMeta<'a>,
{
    Composite::put_input(inner, input_name, input);

    match outer.for_mut_fn() {
        MutFnCtx::Free(_ctx) => Composite::transform(body_mode, inner, body),
        MutFnCtx::Const(ctx) => {
            let eval = |inner: &mut crate::Ctx| Composite::transform(body_mode, inner, body);
            Composite::with_ctx(inner, ctx.unwrap(), ctx_name, Invariant::Const, eval)
        }
        MutFnCtx::Mut(ctx) => {
            let eval = |inner: &mut crate::Ctx| Composite::transform(body_mode, inner, body);
            Composite::with_ctx(inner, ctx.unwrap(), ctx_name, Invariant::Final, eval)
        }
    }
}
