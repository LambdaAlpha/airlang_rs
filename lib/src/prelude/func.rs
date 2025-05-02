use crate::{
    CodeMode,
    ConstFnCtx,
    CtxAccess,
    FuncMode,
    Pair,
    SymbolMode,
    bit::Bit,
    ctx::main::MainCtx,
    func::{
        FuncTrait,
        mode::ModeFunc,
        repr::{
            CONST,
            FREE,
            MUTABLE,
            generate_func,
            parse_func,
            parse_mode,
        },
    },
    mode::{
        id::ID,
        repr::parse,
        united::{
            EVAL_LITERAL,
            EVAL_MOVE,
            EVAL_REF,
            FORM_LITERAL,
            FORM_MOVE,
            FORM_REF,
        },
    },
    prelude::{
        Named,
        Prelude,
        PreludeCtx,
        named_const_fn,
        named_free_fn,
        ref_pair_mode,
    },
    symbol::Symbol,
    val::{
        Val,
        ctx::CtxVal,
        func::FuncVal,
    },
};

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
    pub(crate) call_mode: Named<FuncVal>,
    pub(crate) is_primitive: Named<FuncVal>,
    pub(crate) is_extension: Named<FuncVal>,
    pub(crate) is_cell: Named<FuncVal>,
    pub(crate) is_mode: Named<FuncVal>,
    pub(crate) id: Named<FuncVal>,
    pub(crate) call: Named<FuncVal>,
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
            call_mode: call_mode(),
            is_primitive: is_primitive(),
            is_extension: is_extension(),
            is_cell: is_cell(),
            is_mode: is_mode(),
            id: id(),
            call: call(),
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
        self.call_mode.put(ctx);
        self.is_primitive.put(ctx);
        self.is_extension.put(ctx);
        self.is_cell.put(ctx);
        self.is_mode.put(ctx);
        self.id.put(ctx);
        self.call.put(ctx);
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
    let mode = FuncMode::uni_mode(CodeMode::Form, SymbolMode::Literal);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_form_ref() -> Named<FuncVal> {
    let id = FORM_REF;
    let mode = FuncMode::uni_mode(CodeMode::Form, SymbolMode::Ref);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_form_move() -> Named<FuncVal> {
    let id = FORM_MOVE;
    let mode = FuncMode::uni_mode(CodeMode::Form, SymbolMode::Move);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_eval_literal() -> Named<FuncVal> {
    let id = EVAL_LITERAL;
    let mode = FuncMode::uni_mode(CodeMode::Eval, SymbolMode::Literal);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_eval_ref() -> Named<FuncVal> {
    let id = EVAL_REF;
    let mode = FuncMode::uni_mode(CodeMode::Eval, SymbolMode::Ref);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_eval_move() -> Named<FuncVal> {
    let id = EVAL_MOVE;
    let mode = FuncMode::uni_mode(CodeMode::Eval, SymbolMode::Move);
    let func = ModeFunc::new(mode);
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode() -> Named<FuncVal> {
    let id = "mode";
    let f = fn_mode;
    let call = FuncMode::uni_mode(CodeMode::Form, SymbolMode::Literal);
    let mode = FuncMode { call };
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
    let f = fn_new;
    let call = parse_mode();
    let mode = FuncMode { call };
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
    let f = fn_repr;
    let call = FuncMode::default_mode();
    let mode = FuncMode { call };
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
    let f = fn_ctx_access;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_ctx_access(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let access = match func.ctx_access() {
            CtxAccess::Free => FREE,
            CtxAccess::Const => CONST,
            CtxAccess::Mut => MUTABLE,
        };
        Val::Symbol(Symbol::from_str(access))
    })
}

fn call_mode() -> Named<FuncVal> {
    let id = "function.call_mode";
    let f = fn_call_mode;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_call_mode(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let mode = func.mode().call.clone();
        Val::Func(FuncVal::Mode(ModeFunc::new(mode).into()))
    })
}

fn is_primitive() -> Named<FuncVal> {
    let id = "function.is_primitive";
    let f = fn_is_primitive;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_is_primitive(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let is_primitive = func.is_primitive();
        Val::Bit(Bit::new(is_primitive))
    })
}

fn is_extension() -> Named<FuncVal> {
    let id = "function.is_extension";
    let f = fn_is_extension;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_is_extension(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(primitive) = func.primitive() else {
            return Val::default();
        };
        Val::Bit(Bit::new(primitive.is_extension))
    })
}

fn is_cell() -> Named<FuncVal> {
    let id = "function.is_cell";
    let f = fn_is_cell;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_is_cell(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        Val::Bit(Bit::new(func.is_cell()))
    })
}

fn is_mode() -> Named<FuncVal> {
    let id = "function.is_mode";
    let f = fn_is_mode;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_is_mode(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        Val::Bit(Bit::new(matches!(func, FuncVal::Mode(_))))
    })
}

fn id() -> Named<FuncVal> {
    let id = "function.id";
    let f = fn_id;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_id(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(primitive) = func.primitive() else {
            return Val::default();
        };
        Val::Symbol(primitive.id.clone())
    })
}

fn call() -> Named<FuncVal> {
    let id = "function.call";
    let f = fn_call;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_call(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        func.call()
    })
}

fn ctx() -> Named<FuncVal> {
    let id = "function.context";
    let f = fn_ctx;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_ctx(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(composite) = func.composite() else {
            return Val::default();
        };
        Val::Ctx(CtxVal::from(composite.ctx.clone()))
    })
}
