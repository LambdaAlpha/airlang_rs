use std::mem::take;

use super::MutFn;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::CtxError;
use crate::semantics::ctx::CtxValue;
use crate::semantics::val::Val;
use crate::type_::DynRef;
use crate::type_::Symbol;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct FreeComposite {
    pub(crate) body: Val,
    pub(crate) input_name: Symbol,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct DynComposite {
    pub(crate) free: FreeComposite,
    pub(crate) ctx_name: Symbol,
}

impl FreeComposite {
    pub(super) fn call(&self, cfg: &mut Cfg, inner: &mut Ctx, input: Val) -> Val {
        if put_input(inner, self.input_name.clone(), input).is_err() {
            return Val::default();
        }
        composite_call(cfg, inner, self.body.clone())
    }
}

impl DynComposite {
    pub(super) fn call(
        &self, cfg: &mut Cfg, inner: &mut Ctx, outer: DynRef<Val>, input: Val,
    ) -> Val {
        if put_input(inner, self.free.input_name.clone(), input).is_err() {
            return Val::default();
        }
        let eval = |inner: &mut Ctx| composite_call(cfg, inner, self.free.body.clone());
        with_ctx(inner, outer, self.ctx_name.clone(), eval)
    }
}

pub(crate) fn composite_call(cfg: &mut Cfg, ctx: &mut Ctx, body: Val) -> Val {
    let mut ctx_val = Val::Ctx(take(ctx).into());
    let output = Eval.mut_call(cfg, &mut ctx_val, body);
    let Val::Ctx(ctx_val) = ctx_val else {
        unreachable!("composite_call ctx invariant is broken!!!")
    };
    *ctx = ctx_val.into();
    output
}

fn put_input(inner: &mut Ctx, input_name: Symbol, input: Val) -> Result<(), CtxError> {
    let _ = inner.put(input_name, input, Contract::None)?;
    Ok(())
}

fn with_ctx(
    inner: &mut Ctx, mut outer: DynRef<Val>, name: Symbol, f: impl FnOnce(&mut Ctx) -> Val,
) -> Val {
    if !inner.is_null(name.clone()) {
        return Val::default();
    }
    keep_ctx(inner, outer.reborrow(), name.clone());
    let output = f(inner);
    restore_ctx(inner, outer.unwrap(), name);
    output
}

fn keep_ctx(inner: &mut Ctx, outer: DynRef<Val>, name: Symbol) {
    let const_ = outer.is_const();
    // here is why we need a `&mut Ctx` for a const func
    let outer = take(outer.unwrap());
    let contract = if const_ { Contract::Const } else { Contract::Static };
    let _ = inner.put_unchecked(name, CtxValue::new(outer, contract));
}

fn restore_ctx(inner: &mut Ctx, outer: &mut Val, name: Symbol) {
    let Some(ctx_val) = inner.remove_unchecked(&name) else {
        unreachable!("restore_ctx ctx invariant is broken!!!");
    };
    *outer = ctx_val.val;
}
