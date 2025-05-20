use std::mem::take;

use crate::ConstRef;
use crate::Ctx;
use crate::CtxError;
use crate::CtxVal;
use crate::MutStaticFn;
use crate::Pair;
use crate::Symbol;
use crate::Val;
use crate::ctx::map::CtxValue;
use crate::ctx::map::VarAccess;
use crate::func::func_mode::DEFAULT_MODE;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composite {
    pub(crate) body: Val,
    pub(crate) ctx: Ctx,
    pub(crate) input_name: Symbol,
}

impl Composite {
    pub(crate) fn free_transform(
        inner: &mut Ctx, input_name: Symbol, input: Val, body: Val,
    ) -> Val {
        if Self::put_input(inner, input_name, input).is_err() {
            return Val::default();
        }
        Self::transform(inner, body)
    }

    pub(crate) fn const_transform(
        inner: &mut Ctx, ctx_name: Symbol, outer: ConstRef<Ctx>, input_name: Symbol, input: Val,
        body: Val,
    ) -> Val {
        if Self::put_input(inner, input_name, input).is_err() {
            return Val::default();
        }
        let eval = |inner: &mut Ctx| Self::transform(inner, body);
        Self::with_ctx(inner, outer.unwrap(), ctx_name, VarAccess::Const, eval)
    }

    pub fn mut_transform(
        inner: &mut Ctx, ctx_name: Symbol, outer: &mut Ctx, input_name: Symbol, input: Val,
        body: Val,
    ) -> Val {
        if Self::put_input(inner, input_name, input).is_err() {
            return Val::default();
        }
        let eval = |inner: &mut Ctx| Self::transform(inner, body);
        Self::with_ctx(inner, outer, ctx_name, VarAccess::Mut, eval)
    }

    pub(crate) fn put_input(
        inner: &mut Ctx, input_name: Symbol, input: Val,
    ) -> Result<(), CtxError> {
        let _ = inner.variables_mut().put_value(input_name, VarAccess::Assign, input)?;
        Ok(())
    }

    pub(crate) fn transform(ctx: &mut Ctx, body: Val) -> Val {
        DEFAULT_MODE.mut_static_call(ctx, body)
    }

    pub(crate) fn with_ctx(
        inner: &mut Ctx, outer: &mut Ctx, name: Symbol, access: VarAccess,
        f: impl FnOnce(&mut Ctx) -> Val,
    ) -> Val {
        if !inner.variables().is_assignable(name.clone(), access) {
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
        let ctx_value = CtxValue { val, access, static1: true, free: false };
        let _ = inner.variables_mut().put_unchecked(name, ctx_value);
    }

    fn restore_ctx(inner: &mut Ctx, outer: &mut Ctx, name: Symbol) {
        let Some(CtxValue { val: Val::Ctx(outer_val), .. }) = inner.remove_unchecked(&name) else {
            unreachable!("restore_ctx ctx invariant is broken!!!");
        };
        let outer_val = Ctx::from(outer_val);
        *outer = outer_val;
    }

    pub(crate) fn func_code(&self) -> Val {
        let input = Val::Symbol(self.input_name.clone());
        let output = self.body.clone();
        Val::Pair(Pair::new(input, output).into())
    }

    pub(crate) fn ctx_aware_func_code(ctx: Symbol, input: Symbol, output: Val) -> Val {
        let ctx = Val::Symbol(ctx);
        let input = Val::Symbol(input);
        let names = Val::Pair(Pair::new(ctx, input).into());
        Val::Pair(Pair::new(names, output).into())
    }
}
