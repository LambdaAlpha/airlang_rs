use std::mem::take;

use crate::ConstRef;
use crate::Ctx;
use crate::CtxError;
use crate::MutStaticFn;
use crate::Pair;
use crate::Symbol;
use crate::Val;
use crate::ctx::map::Contract;
use crate::ctx::map::CtxValue;
use crate::func::func_mode::DEFAULT_MODE;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composite {
    pub(crate) body: Val,
    pub(crate) ctx: Ctx,
    pub(crate) input_name: Symbol,
}

impl Composite {
    pub(crate) fn free_call(inner: &mut Ctx, input_name: Symbol, input: Val, body: Val) -> Val {
        if Self::put_input(inner, input_name, input).is_err() {
            return Val::default();
        }
        Self::call(inner, body)
    }

    pub(crate) fn const_call(
        inner: &mut Ctx, ctx_name: Symbol, outer: ConstRef<Val>, input_name: Symbol, input: Val,
        body: Val,
    ) -> Val {
        if Self::put_input(inner, input_name, input).is_err() {
            return Val::default();
        }
        let eval = |inner: &mut Ctx| Self::call(inner, body);
        Self::with_ctx(inner, outer.unwrap(), ctx_name, true, eval)
    }

    pub fn mut_call(
        inner: &mut Ctx, ctx_name: Symbol, outer: &mut Val, input_name: Symbol, input: Val,
        body: Val,
    ) -> Val {
        if Self::put_input(inner, input_name, input).is_err() {
            return Val::default();
        }
        let eval = |inner: &mut Ctx| Self::call(inner, body);
        Self::with_ctx(inner, outer, ctx_name, false, eval)
    }

    pub(crate) fn put_input(
        inner: &mut Ctx, input_name: Symbol, input: Val,
    ) -> Result<(), CtxError> {
        let _ = inner.variables_mut().put(input_name, input, Contract::None)?;
        Ok(())
    }

    pub(crate) fn call(ctx: &mut Ctx, body: Val) -> Val {
        Self::ctx_call(&DEFAULT_MODE, ctx, body)
    }

    pub(crate) fn ctx_call<Fn>(f: &Fn, ctx: &mut Ctx, body: Val) -> Val
    where Fn: MutStaticFn<Val, Val, Val> {
        let mut ctx_val = Val::Ctx(take(ctx).into());
        let output = f.mut_static_call(&mut ctx_val, body);
        let Val::Ctx(ctx_val) = ctx_val else {
            unreachable!("ctx_call ctx invariant is broken!!!")
        };
        *ctx = ctx_val.into();
        output
    }

    pub(crate) fn with_ctx(
        inner: &mut Ctx, outer: &mut Val, name: Symbol, const1: bool,
        f: impl FnOnce(&mut Ctx) -> Val,
    ) -> Val {
        if !inner.variables().is_null(name.clone()) {
            return Val::default();
        }
        Self::keep_ctx(inner, outer, name.clone(), const1);
        let output = f(inner);
        Self::restore_ctx(inner, outer, name);
        output
    }

    fn keep_ctx(inner: &mut Ctx, outer: &mut Val, name: Symbol, const1: bool) {
        // here is why we need a `&mut Ctx` for a const func
        let outer = take(outer);
        let contract = if const1 { Contract::Const } else { Contract::Static };
        let _ = inner.variables_mut().put_unchecked(name, CtxValue::new(outer, contract));
    }

    fn restore_ctx(inner: &mut Ctx, outer: &mut Val, name: Symbol) {
        let Some(ctx_val) = inner.variables_mut().remove_unchecked(&name) else {
            unreachable!("restore_ctx ctx invariant is broken!!!");
        };
        *outer = ctx_val.val;
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
