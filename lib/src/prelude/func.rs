use repr::generate_func;
use repr::mode::EVAL_LITERAL;
use repr::mode::EVAL_MOVE;
use repr::mode::EVAL_REF;
use repr::mode::FORM_LITERAL;
use repr::mode::FORM_MOVE;
use repr::mode::FORM_REF;
use repr::mode::parse;
use repr::parse_func;
use repr::parse_mode;

use super::DynFn;
use super::FreeFn;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::ctx_default_mode;
use super::ctx_put;
use super::free_impl;
use super::func::repr::generate_ctx_access;
use crate::semantics::func::FuncMode;
use crate::semantics::func::FuncTrait;
use crate::semantics::func::ModeFunc;
use crate::semantics::mode::CodeMode;
use crate::semantics::mode::SymbolMode;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::CtxVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct FuncPrelude {
    pub mode_id: FuncVal,
    pub mode_form_literal: FuncVal,
    pub mode_form_ref: FuncVal,
    pub mode_form_move: FuncVal,
    pub mode_eval_literal: FuncVal,
    pub mode_eval_ref: FuncVal,
    pub mode_eval_move: FuncVal,
    pub mode: FreeStaticPrimFuncVal,
    pub new: FreeStaticPrimFuncVal,
    pub repr: FreeStaticPrimFuncVal,
    pub ctx_access: ConstStaticPrimFuncVal,
    pub ctx_explicit: ConstStaticPrimFuncVal,
    pub forward_mode: ConstStaticPrimFuncVal,
    pub reverse_mode: ConstStaticPrimFuncVal,
    pub is_primitive: ConstStaticPrimFuncVal,
    pub is_cell: ConstStaticPrimFuncVal,
    pub is_mode: ConstStaticPrimFuncVal,
    pub id: ConstStaticPrimFuncVal,
    pub code: ConstStaticPrimFuncVal,
    pub ctx: ConstStaticPrimFuncVal,
}

impl Default for FuncPrelude {
    fn default() -> Self {
        FuncPrelude {
            mode_id: mode_id(),
            mode_form_literal: mode_form_literal(),
            mode_form_ref: mode_form_ref(),
            mode_form_move: mode_form_move(),
            mode_eval_literal: mode_eval_literal(),
            mode_eval_ref: mode_eval_ref(),
            mode_eval_move: mode_eval_move(),
            mode: mode(),
            new: new(),
            repr: repr(),
            ctx_access: ctx_access(),
            ctx_explicit: ctx_explicit(),
            forward_mode: forward_mode(),
            reverse_mode: reverse_mode(),
            is_primitive: is_primitive(),
            is_cell: is_cell(),
            is_mode: is_mode(),
            id: id(),
            code: code(),
            ctx: ctx(),
        }
    }
}

impl Prelude for FuncPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        ctx_put(ctx, "id", &self.mode_id);
        ctx_put(ctx, FORM_LITERAL, &self.mode_form_literal);
        ctx_put(ctx, FORM_REF, &self.mode_form_ref);
        ctx_put(ctx, FORM_MOVE, &self.mode_form_move);
        ctx_put(ctx, EVAL_LITERAL, &self.mode_eval_literal);
        ctx_put(ctx, EVAL_REF, &self.mode_eval_ref);
        ctx_put(ctx, EVAL_MOVE, &self.mode_eval_move);
        self.mode.put(ctx);
        self.new.put(ctx);
        self.repr.put(ctx);
        self.ctx_access.put(ctx);
        self.ctx_explicit.put(ctx);
        self.forward_mode.put(ctx);
        self.reverse_mode.put(ctx);
        self.is_primitive.put(ctx);
        self.is_cell.put(ctx);
        self.is_mode.put(ctx);
        self.id.put(ctx);
        self.code.put(ctx);
        self.ctx.put(ctx);
    }
}

pub fn mode_id() -> FuncVal {
    let func = ModeFunc::new(None);
    FuncVal::Mode(func.into())
}

pub fn mode_form_literal() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

pub fn mode_form_ref() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Ref, CodeMode::Form);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

pub fn mode_form_move() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Move, CodeMode::Form);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

pub fn mode_eval_literal() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Eval);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

pub fn mode_eval_ref() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Ref, CodeMode::Eval);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

pub fn mode_eval_move() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Move, CodeMode::Eval);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

pub fn mode() -> FreeStaticPrimFuncVal {
    FreeFn {
        id: "mode",
        f: free_impl(fn_mode),
        mode: FuncMode {
            forward: FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form),
            reverse: FuncMode::default_mode(),
        },
    }
    .free_static()
}

fn fn_mode(input: Val) -> Val {
    let Some(mode) = parse(input) else {
        return Val::default();
    };
    let func = ModeFunc::new(mode);
    Val::Func(FuncVal::Mode(func.into()))
}

pub fn new() -> FreeStaticPrimFuncVal {
    FreeFn {
        id: "function",
        f: free_impl(fn_new),
        mode: FuncMode { forward: parse_mode(), reverse: FuncMode::default_mode() },
    }
    .free_static()
}

fn fn_new(input: Val) -> Val {
    match parse_func(input) {
        Some(func) => Val::Func(func),
        None => Val::default(),
    }
}

pub fn repr() -> FreeStaticPrimFuncVal {
    FreeFn { id: "function.represent", f: free_impl(fn_repr), mode: FuncMode::default() }
        .free_static()
}

fn fn_repr(input: Val) -> Val {
    let Val::Func(func) = input else {
        return Val::default();
    };
    generate_func(func)
}

pub fn ctx_access() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.context_access",
        f: const_impl(fn_ctx_access),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_ctx_access(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let access = generate_ctx_access(func.ctx_access());
    Val::Symbol(Symbol::from_str_unchecked(access))
}

pub fn ctx_explicit() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.is_context_explicit",
        f: const_impl(fn_ctx_explicit),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_ctx_explicit(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    Val::Bit(Bit::new(func.ctx_explicit()))
}

pub fn forward_mode() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.forward_mode",
        f: const_impl(fn_forward_mode),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_forward_mode(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let mode = func.mode().forward.clone();
    Val::Func(FuncVal::Mode(ModeFunc::new(mode).into()))
}

pub fn reverse_mode() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.reverse_mode",
        f: const_impl(fn_reverse_mode),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_reverse_mode(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let mode = func.mode().reverse.clone();
    Val::Func(FuncVal::Mode(ModeFunc::new(mode).into()))
}

pub fn is_primitive() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.is_primitive",
        f: const_impl(fn_is_primitive),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_is_primitive(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let is_primitive = func.is_primitive();
    Val::Bit(Bit::new(is_primitive))
}

pub fn is_cell() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.is_cell",
        f: const_impl(fn_is_cell),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_is_cell(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    Val::Bit(Bit::new(func.is_cell()))
}

pub fn is_mode() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.is_mode",
        f: const_impl(fn_is_mode),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_is_mode(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    Val::Bit(Bit::new(matches!(func, FuncVal::Mode(_))))
}

pub fn id() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.id",
        f: const_impl(fn_id),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_id(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let Some(id) = func.id() else {
        return Val::default();
    };
    Val::Symbol(id)
}

pub fn code() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.code",
        f: const_impl(fn_code),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_code(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    func.code()
}

pub fn ctx() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.context",
        f: const_impl(fn_ctx),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_ctx(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let Some(ctx) = func.ctx() else {
        return Val::default();
    };
    Val::Ctx(CtxVal::from(ctx.clone()))
}

mod repr;
