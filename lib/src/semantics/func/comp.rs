use std::mem::take;
use std::ops::DerefMut;

use super::DynFunc;
use super::PrimCtx;
use super::PrimInput;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::ctx::DynCtx;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::type_::Key;

#[derive(Clone, PartialEq, Eq)]
pub struct CompFunc {
    pub(crate) prelude: Val,
    pub(crate) body: Val,
    pub(crate) ctx: CompCtx,
    pub(crate) input: CompInput,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum CompCtx {
    Free,
    Aware { name: Key, const_: bool },
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum CompInput {
    Free,
    Aware { name: Key },
}

impl DynFunc<Cfg, Val, Val, Val> for CompFunc {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        let new_ctx = &mut self.prelude.clone();
        if let CompInput::Aware { name, .. } = &self.input {
            let set_result = new_ctx.set(cfg, name.clone(), input);
            if set_result.is_none() {
                return Val::default();
            }
        }
        let CompCtx::Aware { name, const_ } = &self.ctx else {
            return Eval.call(cfg, new_ctx, self.body.clone());
        };
        let ctx_link = LinkVal::new(take(ctx), *const_);
        let output = 'output: {
            if new_ctx.set(cfg, name.clone(), Val::Link(ctx_link.clone())).is_none() {
                break 'output Val::default();
            }
            Eval.call(cfg, new_ctx, self.body.clone())
        };
        let mut ctx_updated =
            ctx_link.try_borrow_mut().expect("ctx link should not be borrowed after eval");
        *ctx = take(ctx_updated.deref_mut());
        output
    }
}

impl CompCtx {
    pub(crate) fn to_prim_ctx(&self) -> PrimCtx {
        match self {
            CompCtx::Aware { const_, .. } => {
                if *const_ {
                    PrimCtx::Const_
                } else {
                    PrimCtx::Mut
                }
            },
            CompCtx::Free => PrimCtx::Free,
        }
    }
}

impl CompInput {
    pub(crate) fn to_prim_input(&self) -> PrimInput {
        match self {
            CompInput::Aware { .. } => PrimInput::Aware,
            CompInput::Free => PrimInput::Free,
        }
    }
}
