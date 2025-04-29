use crate::{
    ConstFnCtx,
    MutFnCtx,
    Pair,
    Symbol,
    Val,
    ctx::{
        map::VarAccess,
        ref1::CtxMeta,
    },
    func::comp::Composite,
};

pub(crate) fn const_func_transform<'a, Ctx>(
    inner: &mut crate::Ctx, ctx_name: Symbol, outer: Ctx, input_name: Symbol, input: Val, body: Val,
) -> Val
where Ctx: CtxMeta<'a> {
    if Composite::put_input(inner, input_name, input).is_err() {
        return Val::default();
    }

    match outer.for_const_fn() {
        ConstFnCtx::Free(_ctx) => Composite::transform(inner, body),
        ConstFnCtx::Const(ctx) => {
            let eval = |inner: &mut crate::Ctx| Composite::transform(inner, body);
            Composite::with_ctx(inner, ctx.unwrap(), ctx_name, VarAccess::Const, eval)
        }
    }
}

pub(crate) fn mut_func_transform<'a, Ctx>(
    inner: &mut crate::Ctx, ctx_name: Symbol, outer: Ctx, input_name: Symbol, input: Val, body: Val,
) -> Val
where Ctx: CtxMeta<'a> {
    if Composite::put_input(inner, input_name, input).is_err() {
        return Val::default();
    }

    match outer.for_mut_fn() {
        MutFnCtx::Free(_ctx) => Composite::transform(inner, body),
        MutFnCtx::Const(ctx) => {
            let eval = |inner: &mut crate::Ctx| Composite::transform(inner, body);
            Composite::with_ctx(inner, ctx.unwrap(), ctx_name, VarAccess::Const, eval)
        }
        MutFnCtx::Mut(ctx) => {
            let eval = |inner: &mut crate::Ctx| Composite::transform(inner, body);
            Composite::with_ctx(inner, ctx.unwrap(), ctx_name, VarAccess::Mut, eval)
        }
    }
}

pub(crate) fn func_call(ctx: Symbol, input: Symbol, output: Val) -> Val {
    let ctx = Val::Symbol(ctx);
    let input = Val::Symbol(input);
    let names = Val::Pair(Pair::new(ctx, input).into());
    Val::Pair(Pair::new(names, output).into())
}
