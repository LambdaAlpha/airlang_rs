use crate::CodeMode;
use crate::ConstRef;
use crate::ConstStaticPrimFuncVal;
use crate::CtxAccess;
use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::SymbolMode;
use crate::bit::Bit;
use crate::func::FuncTrait;
use crate::func::mode::ModeFunc;
use crate::func::repr::CONST;
use crate::func::repr::FREE;
use crate::func::repr::MUTABLE;
use crate::func::repr::generate_func;
use crate::func::repr::parse_func;
use crate::func::repr::parse_mode;
use crate::mode::ID;
use crate::mode::prim::EVAL_LITERAL;
use crate::mode::prim::EVAL_MOVE;
use crate::mode::prim::EVAL_REF;
use crate::mode::prim::FORM_LITERAL;
use crate::mode::prim::FORM_MOVE;
use crate::mode::prim::FORM_REF;
use crate::mode::repr::parse;
use crate::prelude::DynFn;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::ctx_put;
use crate::prelude::free_impl;
use crate::symbol::Symbol;
use crate::val::Val;
use crate::val::ctx::CtxVal;
use crate::val::func::FuncVal;

#[derive(Clone)]
pub(crate) struct FuncPrelude {
    pub(crate) mode_id: FuncVal,
    pub(crate) mode_form_literal: FuncVal,
    pub(crate) mode_form_ref: FuncVal,
    pub(crate) mode_form_move: FuncVal,
    pub(crate) mode_eval_literal: FuncVal,
    pub(crate) mode_eval_ref: FuncVal,
    pub(crate) mode_eval_move: FuncVal,
    pub(crate) mode: FreeStaticPrimFuncVal,
    pub(crate) new: FreeStaticPrimFuncVal,
    pub(crate) repr: FreeStaticPrimFuncVal,
    pub(crate) ctx_access: ConstStaticPrimFuncVal,
    pub(crate) ctx_explicit: ConstStaticPrimFuncVal,
    pub(crate) forward_mode: ConstStaticPrimFuncVal,
    pub(crate) reverse_mode: ConstStaticPrimFuncVal,
    pub(crate) is_primitive: ConstStaticPrimFuncVal,
    pub(crate) is_extension: ConstStaticPrimFuncVal,
    pub(crate) is_cell: ConstStaticPrimFuncVal,
    pub(crate) is_mode: ConstStaticPrimFuncVal,
    pub(crate) id: ConstStaticPrimFuncVal,
    pub(crate) code: ConstStaticPrimFuncVal,
    pub(crate) ctx: ConstStaticPrimFuncVal,
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
            is_extension: is_extension(),
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
        ctx_put(ctx, ID, &self.mode_id);
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
        self.is_extension.put(ctx);
        self.is_cell.put(ctx);
        self.is_mode.put(ctx);
        self.id.put(ctx);
        self.code.put(ctx);
        self.ctx.put(ctx);
    }
}

fn mode_id() -> FuncVal {
    let func = ModeFunc::new(None);
    FuncVal::Mode(func.into())
}

fn mode_form_literal() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

fn mode_form_ref() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Ref, CodeMode::Form);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

fn mode_form_move() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Move, CodeMode::Form);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

fn mode_eval_literal() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Eval);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

fn mode_eval_ref() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Ref, CodeMode::Eval);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

fn mode_eval_move() -> FuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Move, CodeMode::Eval);
    let func = ModeFunc::new(mode);
    FuncVal::Mode(func.into())
}

fn mode() -> FreeStaticPrimFuncVal {
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

fn new() -> FreeStaticPrimFuncVal {
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

fn repr() -> FreeStaticPrimFuncVal {
    FreeFn { id: "function.represent", f: free_impl(fn_repr), mode: FuncMode::default() }
        .free_static()
}

fn fn_repr(input: Val) -> Val {
    let Val::Func(func) = input else {
        return Val::default();
    };
    generate_func(func)
}

fn ctx_access() -> ConstStaticPrimFuncVal {
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
    let access = match func.ctx_access() {
        CtxAccess::Free => FREE,
        CtxAccess::Const => CONST,
        CtxAccess::Mut => MUTABLE,
    };
    Val::Symbol(Symbol::from_str(access))
}

fn ctx_explicit() -> ConstStaticPrimFuncVal {
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

fn forward_mode() -> ConstStaticPrimFuncVal {
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

fn reverse_mode() -> ConstStaticPrimFuncVal {
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

fn is_primitive() -> ConstStaticPrimFuncVal {
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

fn is_extension() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.is_extension",
        f: const_impl(fn_is_extension),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_is_extension(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let Some(extension) = func.extension() else {
        return Val::default();
    };
    Val::Bit(Bit::new(extension))
}

fn is_cell() -> ConstStaticPrimFuncVal {
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

fn is_mode() -> ConstStaticPrimFuncVal {
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

fn id() -> ConstStaticPrimFuncVal {
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
    Val::Symbol(id.clone())
}

fn code() -> ConstStaticPrimFuncVal {
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

fn ctx() -> ConstStaticPrimFuncVal {
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
