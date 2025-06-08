use super::DynFn;
use super::ctx::ref_::RefCtx;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::DEFAULT_MODE;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::FuncMode;
use crate::semantics::func::MutStaticFn;
use crate::semantics::func::MutStaticImpl;
use crate::semantics::mode::MOVE_CHAR;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

thread_local!(pub(in crate::prelude) static MODE_PRELUDE: ModePrelude = ModePrelude::default());

#[derive(Clone)]
pub struct ModePrelude {
    pub ref_mode: MutStaticPrimFuncVal,
}

impl Default for ModePrelude {
    fn default() -> Self {
        Self { ref_mode: ref_mode() }
    }
}

// todo design
// todo rename
pub fn ref_mode() -> MutStaticPrimFuncVal {
    DynFn {
        id: "mode.reference",
        f: MutStaticImpl::new(fn_ref_mode_free, fn_ref_mode_const, fn_ref_mode_mut),
        mode: FuncMode::id_func_mode(),
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_ref_mode_free(input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = DEFAULT_MODE.free_static_call(input);
        return RefCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(MOVE_CHAR) = prefix {
        return Val::default();
    }
    input
}

fn fn_ref_mode_const(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = DEFAULT_MODE.const_static_call(ctx, input);
        return RefCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(MOVE_CHAR) = prefix {
        return Val::default();
    }
    input
}

fn fn_ref_mode_mut(ctx: &mut Val, input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = DEFAULT_MODE.mut_static_call(ctx, input);
        return RefCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(MOVE_CHAR) = prefix {
        let Val::Ctx(ctx) = ctx else {
            return Val::default();
        };
        let val =
            ctx.variables_mut().remove(Symbol::from_str_unchecked(&s[1 ..])).unwrap_or_default();
        return RefCtx::escape_symbol(val);
    }
    input
}
