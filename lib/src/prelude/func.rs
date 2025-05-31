use crate::CodeMode;
use crate::ConstRef;
use crate::CtxAccess;
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
use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::free_impl;
use crate::prelude::named_const_fn;
use crate::prelude::named_free_fn;
use crate::symbol::Symbol;
use crate::val::Val;
use crate::val::ctx::CtxVal;
use crate::val::func::FuncVal;

#[derive(Clone)]
pub(crate) struct FuncPrelude {
    pub(crate) mode_id: Named<FuncVal>,
    pub(crate) mode_form_literal: Named<FuncVal>,
    pub(crate) mode_form_ref: Named<FuncVal>,
    pub(crate) mode_form_move: Named<FuncVal>,
    pub(crate) mode_eval_literal: Named<FuncVal>,
    pub(crate) mode_eval_ref: Named<FuncVal>,
    pub(crate) mode_eval_move: Named<FuncVal>,
    pub(crate) mode: Named<FuncVal>,
    pub(crate) new: Named<FuncVal>,
    pub(crate) repr: Named<FuncVal>,
    pub(crate) ctx_access: Named<FuncVal>,
    pub(crate) ctx_explicit: Named<FuncVal>,
    pub(crate) forward_mode: Named<FuncVal>,
    pub(crate) reverse_mode: Named<FuncVal>,
    pub(crate) is_primitive: Named<FuncVal>,
    pub(crate) is_extension: Named<FuncVal>,
    pub(crate) is_cell: Named<FuncVal>,
    pub(crate) is_mode: Named<FuncVal>,
    pub(crate) id: Named<FuncVal>,
    pub(crate) code: Named<FuncVal>,
    pub(crate) ctx: Named<FuncVal>,
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
        self.mode_id.put(ctx);
        self.mode_form_literal.put(ctx);
        self.mode_form_ref.put(ctx);
        self.mode_form_move.put(ctx);
        self.mode_eval_literal.put(ctx);
        self.mode_eval_ref.put(ctx);
        self.mode_eval_move.put(ctx);
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

fn mode_id() -> Named<FuncVal> {
    let id = ID;
    let func = ModeFunc::new(None);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_form_literal() -> Named<FuncVal> {
    let id = FORM_LITERAL;
    let mode = FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_form_ref() -> Named<FuncVal> {
    let id = FORM_REF;
    let mode = FuncMode::prim_mode(SymbolMode::Ref, CodeMode::Form);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_form_move() -> Named<FuncVal> {
    let id = FORM_MOVE;
    let mode = FuncMode::prim_mode(SymbolMode::Move, CodeMode::Form);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_eval_literal() -> Named<FuncVal> {
    let id = EVAL_LITERAL;
    let mode = FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Eval);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_eval_ref() -> Named<FuncVal> {
    let id = EVAL_REF;
    let mode = FuncMode::prim_mode(SymbolMode::Ref, CodeMode::Eval);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_eval_move() -> Named<FuncVal> {
    let id = EVAL_MOVE;
    let mode = FuncMode::prim_mode(SymbolMode::Move, CodeMode::Eval);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode() -> Named<FuncVal> {
    let id = "mode";
    let f = free_impl(fn_mode);
    let forward = FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form);
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_free_fn(id, f, mode)
}

fn fn_mode(input: Val) -> Val {
    let Some(mode) = parse(input) else {
        return Val::default();
    };
    let func = ModeFunc::new(mode);
    Val::Func(FuncVal::Mode(func.into()))
}

fn new() -> Named<FuncVal> {
    let id = "function";
    let f = free_impl(fn_new);
    let forward = parse_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_free_fn(id, f, mode)
}

fn fn_new(input: Val) -> Val {
    match parse_func(input) {
        Some(func) => Val::Func(func),
        None => Val::default(),
    }
}

fn repr() -> Named<FuncVal> {
    let id = "function.represent";
    let f = free_impl(fn_repr);
    let forward = FuncMode::default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_free_fn(id, f, mode)
}

fn fn_repr(input: Val) -> Val {
    let Val::Func(func) = input else {
        return Val::default();
    };
    generate_func(func)
}

fn ctx_access() -> Named<FuncVal> {
    let id = "function.context_access";
    let f = const_impl(fn_ctx_access);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
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

fn ctx_explicit() -> Named<FuncVal> {
    let id = "function.is_context_explicit";
    let f = const_impl(fn_ctx_explicit);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_ctx_explicit(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    Val::Bit(Bit::new(func.ctx_explicit()))
}

fn forward_mode() -> Named<FuncVal> {
    let id = "function.forward_mode";
    let f = const_impl(fn_forward_mode);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_forward_mode(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let mode = func.mode().forward.clone();
    Val::Func(FuncVal::Mode(ModeFunc::new(mode).into()))
}

fn reverse_mode() -> Named<FuncVal> {
    let id = "function.reverse_mode";
    let f = const_impl(fn_reverse_mode);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_reverse_mode(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let mode = func.mode().reverse.clone();
    Val::Func(FuncVal::Mode(ModeFunc::new(mode).into()))
}

fn is_primitive() -> Named<FuncVal> {
    let id = "function.is_primitive";
    let f = const_impl(fn_is_primitive);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_is_primitive(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let is_primitive = func.is_primitive();
    Val::Bit(Bit::new(is_primitive))
}

fn is_extension() -> Named<FuncVal> {
    let id = "function.is_extension";
    let f = const_impl(fn_is_extension);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_is_extension(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let Some(primitive) = func.primitive() else {
        return Val::default();
    };
    Val::Bit(Bit::new(primitive.is_extension))
}

fn is_cell() -> Named<FuncVal> {
    let id = "function.is_cell";
    let f = const_impl(fn_is_cell);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_is_cell(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    Val::Bit(Bit::new(func.is_cell()))
}

fn is_mode() -> Named<FuncVal> {
    let id = "function.is_mode";
    let f = const_impl(fn_is_mode);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_is_mode(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    Val::Bit(Bit::new(matches!(func, FuncVal::Mode(_))))
}

fn id() -> Named<FuncVal> {
    let id = "function.id";
    let f = const_impl(fn_id);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_id(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let Some(primitive) = func.primitive() else {
        return Val::default();
    };
    Val::Symbol(primitive.id.clone())
}

fn code() -> Named<FuncVal> {
    let id = "function.code";
    let f = const_impl(fn_code);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_code(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    func.code()
}

fn ctx() -> Named<FuncVal> {
    let id = "function.context";
    let f = const_impl(fn_ctx);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_ctx(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return Val::default();
    };
    let Some(composite) = func.composite() else {
        return Val::default();
    };
    Val::Ctx(CtxVal::from(composite.ctx.clone()))
}
