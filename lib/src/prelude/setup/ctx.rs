use crate::prelude::MutImpl;
use crate::prelude::ctx::ref_::RefCtx;
use crate::prelude::setup::DynSetupFn;
use crate::semantics::core::Eval;
use crate::semantics::core::SYMBOL_EVAL_CHAR;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct CtxSetup {
    pub ref_: MutPrimFuncVal,
}

impl Default for CtxSetup {
    fn default() -> Self {
        Self { ref_: ref_() }
    }
}

// todo design
// todo rename
pub fn ref_() -> MutPrimFuncVal {
    DynSetupFn { id: "reference", f: MutImpl::new(fn_ref_free, fn_ref_const, fn_ref_mut) }.mut_()
}

fn fn_ref_free(input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = Eval.free_call(input);
        return RefCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(SYMBOL_EVAL_CHAR) = prefix {
        let val = Eval.free_call(Symbol::from_str_unchecked(&s[1 ..]));
        return RefCtx::escape_symbol(val);
    }
    input
}

fn fn_ref_const(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = Eval.const_call(ctx, input);
        return RefCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(SYMBOL_EVAL_CHAR) = prefix {
        let val = Eval.const_call(ctx, Symbol::from_str_unchecked(&s[1 ..]));
        return RefCtx::escape_symbol(val);
    }
    input
}

fn fn_ref_mut(ctx: &mut Val, input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = Eval.mut_call(ctx, input);
        return RefCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(SYMBOL_EVAL_CHAR) = prefix {
        let val = Eval.mut_call(ctx, Symbol::from_str_unchecked(&s[1 ..]));
        return RefCtx::escape_symbol(val);
    }
    input
}
