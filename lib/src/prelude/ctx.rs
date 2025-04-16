use crate::{
    CodeMode,
    FuncMode,
    Map,
    SymbolMode,
    bit::Bit,
    ctx::{
        Ctx,
        const1::{
            ConstCtx,
            ConstFnCtx,
        },
        default::DefaultCtx,
        free::FreeCtx,
        map::{
            CtxMapRef,
            CtxValue,
        },
        mut1::{
            MutCtx,
            MutFnCtx,
        },
        ref1::CtxRef,
        repr::{
            Extra,
            PatternCtx,
            assign_pattern,
            generate_ctx,
            generate_var_access,
            parse_ctx,
            parse_mode,
            parse_pattern,
            parse_var_access,
        },
    },
    mode::eval::EVAL,
    pair::Pair,
    prelude::{
        Named,
        Prelude,
        initial_ctx,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
    },
    symbol::Symbol,
    transformer::Transformer,
    types::either::Either,
    utils::val::symbol,
    val::{
        Val,
        ctx::CtxVal,
        func::FuncVal,
    },
};

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
    pub(crate) get_solver: Named<FuncVal>,
    pub(crate) set_solver: Named<FuncVal>,
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
            get_solver: get_solver(),
            set_solver: set_solver(),
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
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.read.put(m);
        self.move1.put(m);
        self.assign.put(m);
        self.set_variable_access.put(m);
        self.get_variable_access.put(m);
        self.is_null.put(m);
        self.is_static.put(m);
        self.is_reverse.put(m);
        self.set_reverse.put(m);
        self.get_ctx_access.put(m);
        self.get_solver.put(m);
        self.set_solver.put(m);
        self.with_ctx.put(m);
        self.ctx_in_ctx_out.put(m);
        self.ctx_new.put(m);
        self.ctx_repr.put(m);
        self.ctx_prelude.put(m);
        self.ctx_self.put(m);
    }
}

fn read() -> Named<FuncVal> {
    let id = "read";
    let f = fn_read;
    let call = FuncMode::symbol_mode(SymbolMode::Literal);
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_read(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    DefaultCtx::get_or_default(ctx, s)
}

fn move1() -> Named<FuncVal> {
    let id = "move";
    let f = fn_move;
    let call = FuncMode::symbol_mode(SymbolMode::Literal);
    let mode = FuncMode { call };
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
    let call = FuncMode::pair_mode(
        FuncMode::uni_mode(CodeMode::Form, SymbolMode::Literal),
        FuncMode::default_mode(),
    );
    let mode = FuncMode { call };
    named_mut_fn(id, f, mode)
}

fn fn_assign(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = pair.unwrap();
    let pattern_ctx = PatternCtx { extra: Extra::default(), allow_extra: true };
    let Some(pattern) = parse_pattern(pair.first, pattern_ctx) else {
        return Val::default();
    };
    let val = pair.second;
    assign_pattern(ctx, pattern, val)
}

fn set_variable_access() -> Named<FuncVal> {
    let id = "set_variable_access";
    let f = fn_set_variable_access;
    let call = FuncMode::pair_mode(
        FuncMode::symbol_mode(SymbolMode::Literal),
        FuncMode::symbol_mode(SymbolMode::Literal),
    );
    let mode = FuncMode { call };
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
    let call = FuncMode::symbol_mode(SymbolMode::Literal);
    let mode = FuncMode { call };
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
    let call = FuncMode::symbol_mode(SymbolMode::Literal);
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_is_null(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    match DefaultCtx::is_null(ctx, s) {
        Ok(b) => Val::Bit(Bit::new(b)),
        Err(_) => Val::default(),
    }
}

fn is_static() -> Named<FuncVal> {
    let id = "is_static";
    let f = fn_is_static;
    let call = FuncMode::symbol_mode(SymbolMode::Literal);
    let mode = FuncMode { call };
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
    let call = FuncMode::default_mode();
    let mode = FuncMode { call };
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

fn get_solver() -> Named<FuncVal> {
    let id = "solver";
    let f = fn_get_solver;
    let call = FuncMode::default_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_get_solver(ctx: ConstFnCtx, _input: Val) -> Val {
    match ctx.get_solver() {
        Ok(solver) => Val::Func(solver.clone()),
        _ => Val::default(),
    }
}

fn set_solver() -> Named<FuncVal> {
    let id = "set_solver";
    let f = fn_set_solver;
    let mode = FuncMode::default();
    named_mut_fn(id, f, mode)
}

fn fn_set_solver(ctx: MutFnCtx, input: Val) -> Val {
    match input {
        Val::Unit(_) => {
            let _ = ctx.set_solver(None);
        }
        Val::Func(solver) => {
            let _ = ctx.set_solver(Some(solver));
        }
        _ => {}
    }
    Val::default()
}

fn with_ctx() -> Named<FuncVal> {
    let id = "|";
    let f = fn_with_ctx;
    let call = FuncMode::pair_mode(
        FuncMode::id_mode(),
        FuncMode::uni_mode(CodeMode::Form, SymbolMode::Ref),
    );
    let mode = FuncMode { call };
    named_mut_fn(id, f, mode)
}

fn fn_with_ctx(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let val = pair.second;
    DefaultCtx::with_dyn(ctx, pair.first, |ref_or_val| {
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
    let call = FuncMode::pair_mode(
        FuncMode::default_mode(),
        FuncMode::uni_mode(CodeMode::Form, SymbolMode::Ref),
    );
    let mode = FuncMode { call };
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
    let call = parse_mode();
    let mode = FuncMode { call };
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
    let call = FuncMode::default_mode();
    let mode = FuncMode { call };
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
