use log::error;

use crate::prelude::MutStaticImpl;
use crate::prelude::ctx::ref_::RefCtx;
use crate::prelude::setup::DynFn;
use crate::semantics::core::Eval;
use crate::semantics::core::SYMBOL_MOVE_CHAR;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct CtxSetup {
    pub ref_: MutStaticPrimFuncVal,
}

impl Default for CtxSetup {
    fn default() -> Self {
        Self { ref_: ref_() }
    }
}

// todo design
// todo rename
pub fn ref_() -> MutStaticPrimFuncVal {
    DynFn { id: "reference", f: MutStaticImpl::new(fn_ref_free, fn_ref_const, fn_ref_mut) }
        .mut_static()
}

fn fn_ref_free(input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = Eval.free_static_call(input);
        return RefCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(SYMBOL_MOVE_CHAR) = prefix {
        return Val::default();
    }
    input
}

fn fn_ref_const(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = Eval.const_static_call(ctx, input);
        return RefCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(SYMBOL_MOVE_CHAR) = prefix {
        return Val::default();
    }
    input
}

fn fn_ref_mut(ctx: &mut Val, input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = Eval.mut_static_call(ctx, input);
        return RefCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(SYMBOL_MOVE_CHAR) = prefix {
        let Val::Ctx(ctx) = ctx else {
            error!("ctx {ctx:?} should be a ctx");
            return Val::default();
        };
        let val =
            ctx.variables_mut().remove(Symbol::from_str_unchecked(&s[1 ..])).unwrap_or_default();
        return RefCtx::escape_symbol(val);
    }
    input
}
