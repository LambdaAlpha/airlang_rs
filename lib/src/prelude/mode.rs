use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::FuncVal;
use crate::MutStaticFn;
use crate::MutStaticImpl;
use crate::Symbol;
use crate::Val;
use crate::ctx::main::MainCtx;
use crate::mode::symbol::MOVE_CHAR;
use crate::mode::united::DEFAULT_MODE;
use crate::prelude::mut_fn;

thread_local!(pub(crate) static MODE_PRELUDE: ModePrelude = ModePrelude::default());

#[derive(Clone)]
pub(crate) struct ModePrelude {
    pub(crate) ref_mode: FuncVal,
}

impl Default for ModePrelude {
    fn default() -> Self {
        Self { ref_mode: ref_mode() }
    }
}

fn ref_mode() -> FuncVal {
    let id = "mode.reference";
    let f = MutStaticImpl::new(fn_ref_mode_free, fn_ref_mode_const, fn_ref_mode_mut);
    let mode = FuncMode::id_func_mode();
    mut_fn(id, f, mode)
}

fn fn_ref_mode_free(input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = DEFAULT_MODE.free_static_call(input);
        return MainCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(MOVE_CHAR) = prefix {
        return Val::default();
    }
    input
}

fn fn_ref_mode_const(ctx: ConstRef<Ctx>, input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = DEFAULT_MODE.const_static_call(ctx, input);
        return MainCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(MOVE_CHAR) = prefix {
        return Val::default();
    }
    input
}

fn fn_ref_mode_mut(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Symbol(s) = &input else {
        let val = DEFAULT_MODE.mut_static_call(ctx, input);
        return MainCtx::escape_symbol(val);
    };
    let prefix = s.chars().next();
    if let Some(MOVE_CHAR) = prefix {
        let val = MainCtx::remove_or_default(ctx, Symbol::from_str(&s[1 ..]));
        return MainCtx::escape_symbol(val);
    }
    input
}
