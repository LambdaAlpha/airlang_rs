use std::mem::take;

use crate::{
    Ctx,
    CtxVal,
    Mode,
    MutCtx,
    Symbol,
    Val,
    ctx::map::{
        CtxMapRef,
        CtxValue,
        VarAccess,
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
        access: VarAccess,
        f: impl FnOnce(&mut Ctx) -> Val,
    ) -> Val {
        if !inner.variables().is_assignable(name.clone()) {
            return Val::default();
        }
        Self::keep_ctx(inner, outer, name.clone(), access);
        let output = f(inner);
        Self::restore_ctx(inner, outer, name);
        output
    }

    fn keep_ctx(inner: &mut Ctx, outer: &mut Ctx, name: Symbol, access: VarAccess) {
        // here is why we need a `&mut Ctx` for a const func
        let outer = take(outer);
        let val = Val::Ctx(CtxVal::from(outer));
        let old_val = inner.variables_mut().put_unchecked(name, CtxValue {
            val,
            access,
            static1: true,
        });
        assert!(old_val.is_none(), "keep_ctx variable already present");
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
