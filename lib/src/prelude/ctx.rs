use crate::{
    Form,
    FuncMode,
    Map,
    bit::Bit,
    ctx::{
        Ctx,
        CtxValue,
        DynRef,
        const1::{
            ConstCtx,
            ConstFnCtx,
        },
        default::DefaultCtx,
        free::FreeCtx,
        map::CtxMapRef,
        mut1::{
            MutCtx,
            MutFnCtx,
        },
        ref1::CtxRef,
        repr::{
            Extra,
            PatternCtx,
            REVERSE,
            SOLVER,
            VARIABLES,
            assign_pattern,
            generate_ctx,
            generate_invariant,
            parse_ctx,
            parse_invariant,
            parse_pattern,
        },
    },
    mode::{
        Mode,
        eval::EVAL,
    },
    pair::Pair,
    prelude::{
        Named,
        Prelude,
        form_mode,
        initial_ctx,
        map_mode,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        pair_mode,
        symbol_literal_mode,
    },
    symbol::Symbol,
    transformer::Transformer,
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
    pub(crate) set_invariant: Named<FuncVal>,
    pub(crate) get_invariant: Named<FuncVal>,
    pub(crate) is_null: Named<FuncVal>,
    pub(crate) is_static: Named<FuncVal>,
    pub(crate) is_reverse: Named<FuncVal>,
    pub(crate) set_reverse: Named<FuncVal>,
    pub(crate) get_access: Named<FuncVal>,
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
            set_invariant: set_invariant(),
            get_invariant: get_invariant(),
            is_null: is_null(),
            is_static: is_static(),
            is_reverse: is_reverse(),
            set_reverse: set_reverse(),
            get_access: get_access(),
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
        self.set_invariant.put(m);
        self.get_invariant.put(m);
        self.is_null.put(m);
        self.is_static.put(m);
        self.is_reverse.put(m);
        self.set_reverse.put(m);
        self.get_access.put(m);
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
    let call = symbol_literal_mode();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
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
    let call = symbol_literal_mode();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
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
    let call = pair_mode(form_mode(Form::Literal), Mode::default());
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_assign(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = pair.unwrap();
    let pattern_ctx = PatternCtx {
        extra: Extra::default(),
        allow_extra: true,
    };
    let Some(pattern) = parse_pattern(pair.first, pattern_ctx) else {
        return Val::default();
    };
    let val = pair.second;
    assign_pattern(ctx, pattern, val)
}

fn set_invariant() -> Named<FuncVal> {
    let id = "set_invariant";
    let f = fn_set_invariant;
    let call = pair_mode(symbol_literal_mode(), symbol_literal_mode());
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_set_invariant(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        return Val::default();
    };
    let Val::Symbol(invariant) = pair.second else {
        return Val::default();
    };
    let Some(invariant) = parse_invariant(&invariant) else {
        return Val::default();
    };
    let Ok(ctx) = ctx.get_variables_mut() else {
        return Val::default();
    };
    let _ = ctx.set_invariant(s, invariant);
    Val::default()
}

fn get_invariant() -> Named<FuncVal> {
    let id = "invariant";
    let f = fn_get_invariant;
    let call = symbol_literal_mode();
    let abstract1 = call.clone();
    let ask = symbol_literal_mode();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_get_invariant(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let Ok(ctx) = ctx.get_variables() else {
        return Val::default();
    };
    let Some(invariant) = ctx.get_invariant(s) else {
        return Val::default();
    };
    Val::Symbol(generate_invariant(invariant))
}

fn is_null() -> Named<FuncVal> {
    let id = "is_null";
    let f = fn_is_null;
    let call = symbol_literal_mode();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
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
    let call = symbol_literal_mode();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
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
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
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
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
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

fn get_access() -> Named<FuncVal> {
    let id = "access";
    let f = fn_get_access;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = symbol_literal_mode();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

const ACCESS_FREE: &str = "free";
const ACCESS_CONSTANT: &str = "constant";
const ACCESS_MUTABLE: &str = "mutable";

fn fn_get_access(ctx: MutFnCtx, _input: Val) -> Val {
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
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
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
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
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
    let call = pair_mode(symbol_literal_mode(), Mode::default());
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = false;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_with_ctx(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let val = pair.second;
    let target_ctx = match pair.first {
        Val::Unit(_) => MutFnCtx::Free(FreeCtx),
        Val::Symbol(name) => {
            let Ok(ctx) = ctx.get_variables_dyn() else {
                return Val::default();
            };
            let Ok(DynRef { ref1, is_const }) = ctx.ref1.get_ref_dyn(name) else {
                return Val::default();
            };
            let Val::Ctx(target_ctx) = ref1 else {
                return Val::default();
            };
            if ctx.is_const || is_const {
                MutFnCtx::Const(ConstCtx::new(target_ctx))
            } else {
                MutFnCtx::Mut(MutCtx::new(target_ctx))
            }
        }
        _ => return Val::default(),
    };
    EVAL.transform(target_ctx, val)
}

fn ctx_in_ctx_out() -> Named<FuncVal> {
    let id = "|:";
    let f = fn_ctx_in_ctx_out;
    let mode = FuncMode::default();
    let cacheable = false;
    named_free_fn(id, f, mode, cacheable)
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
    let mut map = Map::default();
    map.insert(
        symbol(VARIABLES),
        map_mode(Map::default(), symbol_literal_mode(), Mode::default()),
    );
    map.insert(symbol(REVERSE), Mode::default());
    map.insert(symbol(SOLVER), Mode::default());
    let call = map_mode(map, symbol_literal_mode(), Mode::default());
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
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
    let call = Mode::default();
    let abstract1 = call.clone();
    let mut map = Map::default();
    map.insert(
        symbol(VARIABLES),
        map_mode(Map::default(), symbol_literal_mode(), Mode::default()),
    );
    map.insert(symbol(REVERSE), Mode::default());
    map.insert(symbol(SOLVER), Mode::default());
    let ask = map_mode(map, symbol_literal_mode(), Mode::default());
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
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
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_ctx_prelude(_input: Val) -> Val {
    Val::Ctx(CtxVal::from(initial_ctx()))
}

fn ctx_self() -> Named<FuncVal> {
    let id = "self";
    let f = fn_ctx_self;
    let mode = FuncMode::default();
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_ctx_self(ctx: ConstFnCtx, _input: Val) -> Val {
    let ConstFnCtx::Const(ctx) = ctx else {
        return Val::default();
    };
    let ctx = ctx.unwrap().clone();
    Val::Ctx(CtxVal::from(ctx))
}
