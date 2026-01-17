use std::mem::take;
use std::ops::DerefMut;

use super::MutFn;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::ctx::DynCtx;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::type_::DynRef;
use crate::type_::Key;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct FreeComposite {
    pub(crate) prelude: Val,
    pub(crate) body: Val,
    pub(crate) input_name: Key,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct DynComposite {
    pub(crate) prelude: Val,
    pub(crate) body: Val,
    pub(crate) input_name: Key,
    pub(crate) ctx_name: Key,
}

impl FreeComposite {
    pub(super) fn call(
        cfg: &mut Cfg, input: Val, new_ctx: &mut Val, input_name: Key, body: Val,
    ) -> Val {
        let _ = new_ctx.set(input_name, input);
        Eval.mut_call(cfg, new_ctx, body)
    }
}

impl DynComposite {
    pub(super) fn call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
        let new_ctx = &mut self.prelude.clone();
        let _ = new_ctx.set(self.input_name.clone(), input);
        let const_ = ctx.is_const();
        let ctx = ctx.unwrap();
        let ctx_link = LinkVal::new(take(ctx), const_);
        let _ = new_ctx.set(self.ctx_name.clone(), Val::Link(ctx_link.clone()));
        let output = Eval.mut_call(cfg, new_ctx, self.body.clone());
        let mut new_ctx =
            ctx_link.try_borrow_mut().expect("ctx link should not be borrowed after eval");
        *ctx = take(new_ctx.deref_mut());
        output
    }
}
