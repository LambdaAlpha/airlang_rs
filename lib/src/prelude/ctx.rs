use crate::CodeMode;
use crate::FuncMode;
use crate::SymbolMode;
use crate::bit::Bit;
use crate::ctx::Ctx;
use crate::ctx::const1::ConstCtx;
use crate::ctx::const1::ConstFnCtx;
use crate::ctx::free::FreeCtx;
use crate::ctx::main::MainCtx;
use crate::ctx::map::CtxMapRef;
use crate::ctx::mut1::MutCtx;
use crate::ctx::mut1::MutFnCtx;
use crate::ctx::pattern::PatternCtx;
use crate::ctx::pattern::assign_pattern;
use crate::ctx::pattern::match_pattern;
use crate::ctx::pattern::parse_pattern;
use crate::ctx::ref1::CtxRef;
use crate::ctx::repr::generate_ctx;
use crate::ctx::repr::generate_var_access;
use crate::ctx::repr::parse_ctx;
use crate::ctx::repr::parse_mode;
use crate::ctx::repr::parse_var_access;
use crate::either::Either;
use crate::mode::eval::EVAL;
use crate::pair::Pair;
use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::initial_ctx;
use crate::prelude::named_const_fn;
use crate::prelude::named_free_fn;
use crate::prelude::named_mut_fn;
use crate::transformer::Transformer;
use crate::utils::val::symbol;
use crate::val::Val;
use crate::val::ctx::CtxVal;
use crate::val::func::FuncVal;

#[derive(Clone)]
pub(crate) struct CtxPrelude {
    pub(crate) read: Named<FuncVal>,
    pub(crate) move1: Named<FuncVal>,
    pub(crate) assign: Named<FuncVal>,
    pub(crate) set_variable_access: Named<FuncVal>,
    pub(crate) get_variable_access: Named<FuncVal>,
    pub(crate) is_null: Named<FuncVal>,
    pub(crate) is_static: Named<FuncVal>,
    pub(crate) is_reverse: Named<FuncVal>,
    pub(crate) set_reverse: Named<FuncVal>,
    pub(crate) get_ctx_access: Named<FuncVal>,
    pub(crate) with_ctx: Named<FuncVal>,
    pub(crate) ctx_in_ctx_out: Named<FuncVal>,
    pub(crate) ctx_new: Named<FuncVal>,
    pub(crate) ctx_repr: Named<FuncVal>,
    pub(crate) ctx_prelude: Named<FuncVal>,
    pub(crate) ctx_self: Named<FuncVal>,
}

impl Default for CtxPrelude {
    fn default() -> Self {
        CtxPrelude {
            read: read(),
            move1: move1(),
            assign: assign(),
            set_variable_access: set_variable_access(),
            get_variable_access: get_variable_access(),
            is_null: is_null(),
            is_static: is_static(),
            is_reverse: is_reverse(),
            set_reverse: set_reverse(),
            get_ctx_access: get_ctx_access(),
            with_ctx: with_ctx(),
            ctx_in_ctx_out: ctx_in_ctx_out(),
            ctx_new: ctx_new(),
            ctx_repr: ctx_repr(),
            ctx_prelude: ctx_prelude(),
            ctx_self: ctx_self(),
        }
    }
}

impl Prelude for CtxPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.read.put(ctx);
        self.move1.put(ctx);
        self.assign.put(ctx);
        self.set_variable_access.put(ctx);
        self.get_variable_access.put(ctx);
        self.is_null.put(ctx);
        self.is_static.put(ctx);
        self.is_reverse.put(ctx);
        self.set_reverse.put(ctx);
        self.get_ctx_access.put(ctx);
        self.with_ctx.put(ctx);
        self.ctx_in_ctx_out.put(ctx);
        self.ctx_new.put(ctx);
        self.ctx_repr.put(ctx);
        self.ctx_prelude.put(ctx);
        self.ctx_self.put(ctx);
    }
}

fn read() -> Named<FuncVal> {
    let id = "read";
    let f = fn_read;
    let forward = FuncMode::symbol_mode(SymbolMode::Literal);
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_const_fn(id, f, mode)
}

fn fn_read(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    MainCtx::get_or_default(ctx, s)
}

fn move1() -> Named<FuncVal> {
    let id = "move";
    let f = fn_move;
    let forward = FuncMode::symbol_mode(SymbolMode::Literal);
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_mut_fn(id, f, mode)
}

fn fn_move(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let Ok(ctx) = ctx.get_variables_mut() else {
        return Val::default();
    };
    ctx.remove(s).unwrap_or_default()
}

fn assign() -> Named<FuncVal> {
    let id = "=";
    let f = fn_assign;
    let forward = FuncMode::pair_mode(
        FuncMode::uni_mode(CodeMode::Form, SymbolMode::Literal),
        FuncMode::default_mode(),
    );
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_mut_fn(id, f, mode)
}

fn fn_assign(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = pair.unwrap();
    let pattern_ctx = PatternCtx::default();
    let Some(pattern) = parse_pattern(pattern_ctx, pair.first) else {
        return Val::default();
    };
    let val = pair.second;
    if match_pattern(&pattern, &val) { assign_pattern(ctx, pattern, val) } else { Val::default() }
}

fn set_variable_access() -> Named<FuncVal> {
    let id = "set_variable_access";
    let f = fn_set_variable_access;
    let forward = FuncMode::pair_mode(
        FuncMode::symbol_mode(SymbolMode::Literal),
        FuncMode::symbol_mode(SymbolMode::Literal),
    );
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_mut_fn(id, f, mode)
}

fn fn_set_variable_access(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        return Val::default();
    };
    let Val::Symbol(access) = pair.second else {
        return Val::default();
    };
    let Some(access) = parse_var_access(&access) else {
        return Val::default();
    };
    let Ok(ctx) = ctx.get_variables_mut() else {
        return Val::default();
    };
    let _ = ctx.set_access(s, access);
    Val::default()
}

fn get_variable_access() -> Named<FuncVal> {
    let id = "variable_access";
    let f = fn_get_variable_access;
    let forward = FuncMode::symbol_mode(SymbolMode::Literal);
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_const_fn(id, f, mode)
}

fn fn_get_variable_access(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let Ok(ctx) = ctx.get_variables() else {
        return Val::default();
    };
    let Some(access) = ctx.get_access(s) else {
        return Val::default();
    };
    Val::Symbol(generate_var_access(access))
}

fn is_null() -> Named<FuncVal> {
    let id = "is_null";
    let f = fn_is_null;
    let forward = FuncMode::symbol_mode(SymbolMode::Literal);
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_const_fn(id, f, mode)
}

fn fn_is_null(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    match MainCtx::is_null(ctx, s) {
        Ok(b) => Val::Bit(Bit::new(b)),
        Err(_) => Val::default(),
    }
}

fn is_static() -> Named<FuncVal> {
    let id = "is_static";
    let f = fn_is_static;
    let forward = FuncMode::symbol_mode(SymbolMode::Literal);
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_const_fn(id, f, mode)
}

fn fn_is_static(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let Ok(ctx) = ctx.get_variables() else {
        return Val::default();
    };
    let Some(is_static) = ctx.is_static(s) else {
        return Val::default();
    };
    Val::Bit(Bit::new(is_static))
}

fn is_reverse() -> Named<FuncVal> {
    let id = "is_reverse";
    let f = fn_is_reverse;
    let mode = FuncMode::default();
    named_const_fn(id, f, mode)
}

fn fn_is_reverse(ctx: ConstFnCtx, _input: Val) -> Val {
    let Ok(variables) = ctx.get_variables() else {
        return Val::default();
    };
    Val::Bit(Bit::new(variables.is_reverse()))
}

fn set_reverse() -> Named<FuncVal> {
    let id = "set_reverse";
    let f = fn_set_reverse;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_set_reverse(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Ctx(mut ctx) = pair.first else {
        return Val::default();
    };
    let Val::Bit(bit) = pair.second else {
        return Val::default();
    };
    ctx.variables_mut().set_reverse(bit.bool());
    Val::Ctx(ctx)
}

fn get_ctx_access() -> Named<FuncVal> {
    let id = "access";
    let f = fn_get_ctx_access;
    let forward = FuncMode::default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_mut_fn(id, f, mode)
}

const ACCESS_FREE: &str = "free";
const ACCESS_CONSTANT: &str = "constant";
const ACCESS_MUTABLE: &str = "mutable";

fn fn_get_ctx_access(ctx: MutFnCtx, _input: Val) -> Val {
    let access = match ctx {
        MutFnCtx::Free(_) => ACCESS_FREE,
        MutFnCtx::Const(_) => ACCESS_CONSTANT,
        MutFnCtx::Mut(_) => ACCESS_MUTABLE,
    };
    symbol(access)
}

fn with_ctx() -> Named<FuncVal> {
    let id = "|";
    let f = fn_with_ctx;
    let forward = FuncMode::pair_mode(
        FuncMode::id_mode(),
        FuncMode::uni_mode(CodeMode::Form, SymbolMode::Ref),
    );
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_mut_fn(id, f, mode)
}

fn fn_with_ctx(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let val = pair.second;
    MainCtx::with_dyn(ctx, pair.first, |ref_or_val| {
        let target_ctx = match ref_or_val {
            Either::This(dyn_ref) => {
                let Val::Ctx(target_ctx) = dyn_ref.ref1 else {
                    return Val::default();
                };
                if dyn_ref.is_const {
                    MutFnCtx::Const(ConstCtx::new(target_ctx))
                } else {
                    MutFnCtx::Mut(MutCtx::new(target_ctx))
                }
            }
            Either::That(val) => {
                if !val.is_unit() {
                    return Val::default();
                }
                MutFnCtx::Free(FreeCtx)
            }
        };
        EVAL.transform(target_ctx, val)
    })
}

fn ctx_in_ctx_out() -> Named<FuncVal> {
    let id = "|:";
    let f = fn_ctx_in_ctx_out;
    let forward = FuncMode::pair_mode(
        FuncMode::default_mode(),
        FuncMode::uni_mode(CodeMode::Form, SymbolMode::Ref),
    );
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_free_fn(id, f, mode)
}

fn fn_ctx_in_ctx_out(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let ctx_input = Pair::from(pair);
    let Val::Ctx(ctx) = ctx_input.first else {
        return Val::default();
    };
    let mut ctx = Ctx::from(ctx);
    let input = ctx_input.second;
    let output = EVAL.transform(MutCtx::new(&mut ctx), input);
    let pair = Pair::new(Val::Ctx(ctx.into()), output);
    Val::Pair(pair.into())
}

fn ctx_new() -> Named<FuncVal> {
    let id = "context";
    let f = fn_ctx_new;
    let forward = parse_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_free_fn(id, f, mode)
}

fn fn_ctx_new(input: Val) -> Val {
    match parse_ctx(input) {
        Some(ctx) => Val::Ctx(ctx),
        None => Val::default(),
    }
}

fn ctx_repr() -> Named<FuncVal> {
    let id = "context.represent";
    let f = fn_ctx_repr;
    let forward = FuncMode::default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_free_fn(id, f, mode)
}

fn fn_ctx_repr(input: Val) -> Val {
    let Val::Ctx(ctx) = input else {
        return Val::default();
    };
    generate_ctx(ctx)
}

fn ctx_prelude() -> Named<FuncVal> {
    let id = "prelude";
    let f = fn_ctx_prelude;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_ctx_prelude(_input: Val) -> Val {
    Val::Ctx(CtxVal::from(initial_ctx()))
}

fn ctx_self() -> Named<FuncVal> {
    let id = "self";
    let f = fn_ctx_self;
    let mode = FuncMode::default();
    named_const_fn(id, f, mode)
}

fn fn_ctx_self(ctx: ConstFnCtx, _input: Val) -> Val {
    let ConstFnCtx::Const(ctx) = ctx else {
        return Val::default();
    };
    let ctx = ctx.unwrap().clone();
    Val::Ctx(CtxVal::from(ctx))
}
