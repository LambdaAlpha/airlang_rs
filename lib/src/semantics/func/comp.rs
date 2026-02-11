use std::mem::take;
use std::ops::DerefMut;

use super::DynFunc;
use super::PrimCtx;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::ctx::DynCtx;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::type_::Key;

#[derive(Clone, PartialEq, Eq)]
pub struct CompFunc {
    pub(crate) raw_input: bool,
    pub(crate) fn_: CompFn,
}

impl DynFunc<Cfg, Val, Val, Val> for CompFunc {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        self.fn_.call(cfg, ctx, input)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct CompFn {
    pub(crate) prelude: Val,
    pub(crate) body: Val,
    pub(crate) input_name: Key,
    pub(crate) ctx: CompCtx,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum CompCtx {
    Free,
    Default { name: Key, const_: bool },
}

impl DynFunc<Cfg, Val, Val, Val> for CompFn {
    // todo design support ignore context or input
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        let new_ctx = &mut self.prelude.clone();
        if new_ctx.set(cfg, self.input_name.clone(), input).is_none() {
            return Val::default();
        }
        let CompCtx::Default { name, const_ } = &self.ctx else {
            return Eval.call(cfg, new_ctx, self.body.clone());
        };
        let ctx_link = LinkVal::new(take(ctx), *const_);
        let output = 'output: {
            if new_ctx.set(cfg, name.clone(), Val::Link(ctx_link.clone())).is_none() {
                break 'output Val::default();
            }
            Eval.call(cfg, new_ctx, self.body.clone())
        };
        let mut new_ctx =
            ctx_link.try_borrow_mut().expect("ctx link should not be borrowed after eval");
        *ctx = take(new_ctx.deref_mut());
        output
    }
}

impl CompCtx {
    pub(crate) fn to_prim_ctx(&self) -> PrimCtx {
        match self {
            CompCtx::Default { const_, .. } => {
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
