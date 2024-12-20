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
    pub(crate) ctx: Ctx,
    pub(crate) input_name: Symbol,
}

impl Composite {
    pub(crate) fn put_input(inner: &mut Ctx, input_name: Symbol, input: Val) {
        let _ = inner
            .variables_mut()
            .put_value(input_name, CtxValue::new(input));
    }

    pub(crate) fn transform(mode: &Mode, ctx: &mut Ctx, body: Val) -> Val {
        mode.transform(MutCtx::new(ctx), body)
    }

    pub(crate) fn with_ctx(
        inner: &mut Ctx,
        outer: &mut Ctx,
        name: Symbol,
        invariant: Invariant,
        f: impl FnOnce(&mut Ctx) -> Val,
    ) -> Val {
        if !Self::can_keep(inner, name.clone()) {
            return f(inner);
        }
        Self::keep_ctx(inner, outer, name.clone(), invariant);
        let output = f(inner);
        Self::restore_ctx(inner, outer, name);
        output
    }

    fn can_keep(ctx: &Ctx, name: Symbol) -> bool {
        !ctx.variables().fallback() && ctx.variables().is_assignable(name)
    }

    fn keep_ctx(inner: &mut Ctx, outer: &mut Ctx, name: Symbol, invariant: Invariant) {
        // here is why we need a `&mut Ctx` for a const func
        let outer = take(outer);
        let val = Val::Ctx(CtxVal::from(outer));
        inner
            .variables_mut()
            .put_value(name, CtxValue { val, invariant })
            .expect("name should be assignable");
    }

    fn restore_ctx(inner: &mut Ctx, outer: &mut Ctx, name: Symbol) {
        let Some(CtxValue {
            val: Val::Ctx(outer_val),
            ..
        }) = inner.remove_unchecked(&name)
        else {
            unreachable!("restore_ctx ctx invariant is broken!!!");
        };
        let outer_val = Ctx::from(outer_val);
        *outer = outer_val;
    }
}
