use std::mem::take;
use std::ops::DerefMut;

use super::DynFunc;
use super::PrimCtx;
use super::PrimInput;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::DynCtx;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::type_::Key;

#[derive(Clone)]
pub struct CompFunc {
    pub(crate) prelude: Val,
    pub(crate) body: Val,
    pub(crate) ctx: CompCtx,
    pub(crate) input: CompInput,
}

#[derive(Clone)]
pub(crate) struct CompCtx {
    pub(crate) name: Key,
    pub(crate) prim: PrimCtx,
}

#[derive(Clone)]
pub(crate) struct CompInput {
    pub(crate) name: Key,
    pub(crate) prim: PrimInput,
}

impl DynFunc<Cfg, Val, Val, Val> for CompFunc {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
        let mut new_ctx = self.prelude.clone();
        let new_ctx = Ctx::new_mut(&mut new_ctx);
        if matches!(self.input.prim, PrimInput::Default) {
            let set_result = new_ctx.val.set(cfg, self.input.name.clone(), input);
            if set_result.is_none() {
                return Val::default();
            }
        }
        let const_ = match self.ctx.prim {
            PrimCtx::Free => return Eval.call(cfg, new_ctx, self.body.clone()),
            PrimCtx::Const_ => true,
            PrimCtx::Mut => {
                if ctx.const_ {
                    cfg.abort();
                    return Val::default();
                }
                false
            },
            PrimCtx::Default => ctx.const_,
        };
        let ctx_link = LinkVal::new(take(ctx.val), const_);
        let output = 'output: {
            if new_ctx.val.set(cfg, self.ctx.name.clone(), Val::Link(ctx_link.clone())).is_none() {
                break 'output Val::default();
            }
            Eval.call(cfg, new_ctx, self.body.clone())
        };
        let mut ctx_updated =
            ctx_link.try_borrow_mut().expect("ctx link should not be borrowed after eval");
        *ctx.val = take(ctx_updated.deref_mut());
        output
    }
}
