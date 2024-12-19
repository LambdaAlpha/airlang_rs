use std::mem::take;

use crate::{
    Ctx,
    CtxVal,
    Invariant,
    Mode,
    MutCtx,
    Symbol,
    Val,
    ctx::{
        CtxValue,
        map::CtxMapRef,
    },
    transformer::Transformer,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composite {
    pub(crate) body_mode: Mode,
    pub(crate) body: Val,
    pub(crate) prelude: Ctx,
    pub(crate) input_name: Symbol,
}

pub(crate) fn eval_free(
    prelude: &mut Ctx,
    input: Val,
    input_name: Symbol,
    mode: &Mode,
    body: Val,
) -> Val {
    let _ = prelude
        .variables_mut()
        .put_value(input_name, CtxValue::new(input));
    mode.transform(MutCtx::new(prelude), body)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn eval_aware(
    mut prelude: Ctx,
    ctx: &mut Ctx,
    ctx_name: Symbol,
    ctx_invariant: Invariant,
    input: Val,
    input_name: Symbol,
    mode: &Mode,
    body: Val,
) -> Val {
    let _ = prelude
        .variables_mut()
        .put_value(input_name, CtxValue::new(input));
    keep_eval_restore(prelude, ctx, ctx_name, ctx_invariant, mode, body)
}

fn keep_eval_restore(
    mut prelude: Ctx,
    ctx: &mut Ctx,
    ctx_name: Symbol,
    ctx_invariant: Invariant,
    mode: &Mode,
    body: Val,
) -> Val {
    if !prelude.variables().fallback() && prelude.variables().is_assignable(ctx_name.clone()) {
        // here is why we need a `&mut Ctx` for a const func
        let caller = take(ctx);
        keep_ctx(&mut prelude, caller, ctx_name.clone(), ctx_invariant);
        let output = mode.transform(MutCtx::new(&mut prelude), body);
        restore_ctx(prelude, ctx, ctx_name);
        output
    } else {
        mode.transform(MutCtx::new(&mut prelude), body)
    }
}

fn keep_ctx(prelude: &mut Ctx, ctx: Ctx, name: Symbol, invariant: Invariant) {
    let val = Val::Ctx(CtxVal::from(ctx));
    prelude
        .variables_mut()
        .put_value(name, CtxValue { val, invariant })
        .expect("name should be assignable");
}

fn restore_ctx(prelude: Ctx, ctx: &mut Ctx, name: Symbol) {
    let Ok(Val::Ctx(caller)) = prelude.into_val(name) else {
        unreachable!("restore_ctx ctx invariant is broken!!!");
    };
    let caller = Ctx::from(caller);
    *ctx = caller;
}
