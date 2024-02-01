use crate::{
    bool::Bool,
    ctx::{
        Ctx,
        DefaultCtx,
        NameMap,
    },
    ctx_access::constant::CtxForConstFn,
    eval_mode::EvalMode,
    func::{
        Composed,
        CtxConstEval,
        CtxConstInfo,
        CtxFreeEval,
        CtxFreeInfo,
        CtxMutableEval,
        CtxMutableInfo,
        Func,
        FuncEval,
        FuncImpl,
    },
    io_mode::IoMode,
    map::Map,
    prelude::{
        default_mode,
        map_mode_for_some,
        named_const_fn,
        named_free_fn,
        symbol_value_mode,
        utils::{
            generate_io_mode,
            map_remove,
            parse_io_mode,
            symbol,
        },
        Named,
        Prelude,
    },
    symbol::Symbol,
    types::refer::Reader,
    val::{
        ctx::CtxVal,
        func::FuncVal,
        map::MapVal,
        Val,
    },
};

#[derive(Clone)]
pub(crate) struct FuncPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) repr: Named<FuncVal>,
    pub(crate) caller_access: Named<FuncVal>,
    pub(crate) input_mode: Named<FuncVal>,
    pub(crate) output_mode: Named<FuncVal>,
    pub(crate) is_primitive: Named<FuncVal>,
    pub(crate) is_extension: Named<FuncVal>,
    pub(crate) id: Named<FuncVal>,
    pub(crate) body: Named<FuncVal>,
    pub(crate) ctx: Named<FuncVal>,
    pub(crate) input_name: Named<FuncVal>,
    pub(crate) caller_name: Named<FuncVal>,
}

impl Default for FuncPrelude {
    fn default() -> Self {
        FuncPrelude {
            new: new(),
            repr: repr(),
            caller_access: caller_access(),
            input_mode: input_mode(),
            output_mode: output_mode(),
            is_primitive: is_primitive(),
            is_extension: is_extension(),
            id: id(),
            body: body(),
            ctx: ctx(),
            input_name: input_name(),
            caller_name: caller_name(),
        }
    }
}

impl Prelude for FuncPrelude {
    fn put(&self, m: &mut NameMap) {
        self.new.put(m);
        self.repr.put(m);
        self.caller_access.put(m);
        self.input_mode.put(m);
        self.output_mode.put(m);
        self.is_primitive.put(m);
        self.is_extension.put(m);
        self.id.put(m);
        self.body.put(m);
        self.ctx.put(m);
        self.input_name.put(m);
        self.caller_name.put(m);
    }
}

const BODY: &str = "body";
const CTX: &str = "context";
const INPUT_NAME: &str = "input_name";
const CALLER_NAME: &str = "caller_name";
const ID: &str = "id";
const INPUT_MODE: &str = "input_mode";
const OUTPUT_MODE: &str = "output_mode";
const CALLER_ACCESS: &str = "caller_access";

const DEFAULT_INPUT_NAME: &str = "input";
const DEFAULT_CALLER_NAME: &str = "caller";
const FREE: &str = "free";
const CONST: &str = "constant";
const MUTABLE: &str = "mutable";

fn new() -> Named<FuncVal> {
    let mut map = Map::default();
    map.insert(symbol(BODY), IoMode::Eval(EvalMode::Lazy));
    map.insert(symbol(CTX), default_mode());
    map.insert(symbol(INPUT_NAME), symbol_value_mode());
    map.insert(symbol(CALLER_NAME), symbol_value_mode());
    map.insert(symbol(CALLER_ACCESS), symbol_value_mode());
    map.insert(symbol(INPUT_MODE), IoMode::Eval(EvalMode::Lazy));
    map.insert(symbol(OUTPUT_MODE), IoMode::Eval(EvalMode::Lazy));
    let input_mode = map_mode_for_some(map);
    let output_mode = default_mode();
    named_free_fn("function", input_mode, output_mode, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let body = map_remove(&mut map, BODY);
    let func_ctx = match map_remove(&mut map, CTX) {
        Val::Ctx(func_ctx) => *func_ctx.0,
        Val::Unit(_) => Ctx::default(),
        _ => return Val::default(),
    };
    let input_name = match map_remove(&mut map, INPUT_NAME) {
        Val::Symbol(name) => name,
        Val::Unit(_) => Symbol::from_str(DEFAULT_INPUT_NAME),
        _ => return Val::default(),
    };
    let input_mode = map
        .remove(&symbol(INPUT_MODE))
        .unwrap_or(Val::Bool(Bool::t()));
    let Some(input_mode) = parse_io_mode(input_mode) else {
        return Val::default();
    };
    let output_mode = map
        .remove(&symbol(OUTPUT_MODE))
        .unwrap_or(Val::Bool(Bool::t()));
    let Some(output_mode) = parse_io_mode(output_mode) else {
        return Val::default();
    };
    let caller_name = match map_remove(&mut map, CALLER_NAME) {
        Val::Symbol(name) => name,
        Val::Unit(_) => Symbol::from_str(DEFAULT_CALLER_NAME),
        _ => return Val::default(),
    };
    let caller_access = map_remove(&mut map, CALLER_ACCESS);
    let caller_access = match &caller_access {
        Val::Symbol(s) => &**s,
        Val::Unit(_) => FREE,
        _ => return Val::default(),
    };
    let evaluator = match caller_access {
        FREE => FuncEval::Free(FuncImpl::Composed(Composed {
            body,
            ctx: func_ctx,
            input_name,
            caller: CtxFreeInfo {},
        })),
        CONST => FuncEval::Const(FuncImpl::Composed(Composed {
            body,
            ctx: func_ctx,
            input_name,
            caller: CtxConstInfo { name: caller_name },
        })),
        MUTABLE => FuncEval::Mutable(FuncImpl::Composed(Composed {
            body,
            ctx: func_ctx,
            input_name,
            caller: CtxMutableInfo { name: caller_name },
        })),
        _ => return Val::default(),
    };
    let func = Func::new(input_mode, output_mode, evaluator);
    Val::Func(Reader::new(func).into())
}

fn repr() -> Named<FuncVal> {
    let input_mode = default_mode();
    let mut map = Map::default();
    map.insert(symbol(BODY), IoMode::Eval(EvalMode::Lazy));
    map.insert(symbol(CTX), default_mode());
    map.insert(symbol(INPUT_NAME), symbol_value_mode());
    map.insert(symbol(CALLER_NAME), symbol_value_mode());
    map.insert(symbol(CALLER_ACCESS), symbol_value_mode());
    map.insert(symbol(INPUT_MODE), IoMode::Eval(EvalMode::Lazy));
    map.insert(symbol(OUTPUT_MODE), IoMode::Eval(EvalMode::Lazy));
    map.insert(symbol(ID), IoMode::Eval(EvalMode::Lazy));
    let output_mode = map_mode_for_some(map);
    named_free_fn("function.represent", input_mode, output_mode, fn_repr)
}

fn fn_repr(input: Val) -> Val {
    let Val::Func(FuncVal(func)) = input else {
        return Val::default();
    };
    let mut repr = MapVal::default();

    if func.input_mode != IoMode::default() {
        let input_mode = generate_io_mode(&func.input_mode);
        repr.insert(symbol(INPUT_MODE), input_mode);
    }
    if func.output_mode != IoMode::default() {
        let output_mode = generate_io_mode(&func.output_mode);
        repr.insert(symbol(OUTPUT_MODE), output_mode);
    }

    match &func.evaluator {
        FuncEval::Free(eval) => match eval {
            CtxFreeEval::Primitive(p) => {
                repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
            }
            CtxFreeEval::Composed(c) => {
                repr.insert(symbol(BODY), c.body.clone());
                if c.ctx != Ctx::default() {
                    repr.insert(symbol(CTX), Val::Ctx(CtxVal(Box::new(c.ctx.clone()))));
                }
                if &*c.input_name != DEFAULT_INPUT_NAME {
                    repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                }
            }
        },
        FuncEval::Const(eval) => {
            repr.insert(symbol(CALLER_ACCESS), symbol(CONST));
            match eval {
                CtxConstEval::Primitive(p) => {
                    repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
                }
                CtxConstEval::Composed(c) => {
                    repr.insert(symbol(BODY), c.body.clone());
                    if c.ctx != Ctx::default() {
                        repr.insert(symbol(CTX), Val::Ctx(CtxVal(Box::new(c.ctx.clone()))));
                    }
                    if &*c.input_name != DEFAULT_INPUT_NAME {
                        repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                    }
                    if &*c.caller.name != DEFAULT_CALLER_NAME {
                        repr.insert(symbol(CALLER_NAME), Val::Symbol(c.caller.name.clone()));
                    }
                }
            }
        }
        FuncEval::Mutable(eval) => {
            repr.insert(symbol(CALLER_ACCESS), symbol(MUTABLE));
            match eval {
                CtxMutableEval::Primitive(p) => {
                    repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
                }
                CtxMutableEval::Composed(c) => {
                    repr.insert(symbol(BODY), c.body.clone());
                    if c.ctx != Ctx::default() {
                        repr.insert(symbol(CTX), Val::Ctx(CtxVal(Box::new(c.ctx.clone()))));
                    }
                    if &*c.input_name != DEFAULT_INPUT_NAME {
                        repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                    }
                    if &*c.caller.name != DEFAULT_CALLER_NAME {
                        repr.insert(symbol(CALLER_NAME), Val::Symbol(c.caller.name.clone()));
                    }
                }
            }
        }
    }
    Val::Map(repr)
}

fn caller_access() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = symbol_value_mode();
    named_const_fn(
        "function.caller_access",
        input_mode,
        output_mode,
        fn_caller_access,
    )
}

fn fn_caller_access(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let access = match &func.evaluator {
            FuncEval::Free(_) => FREE,
            FuncEval::Const(_) => CONST,
            FuncEval::Mutable(_) => MUTABLE,
        };
        Val::Symbol(Symbol::from_str(access))
    })
}

fn input_mode() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = IoMode::Eval(EvalMode::Lazy);
    named_const_fn(
        "function.input_mode",
        input_mode,
        output_mode,
        fn_input_mode,
    )
}

fn fn_input_mode(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        generate_io_mode(&func.input_mode)
    })
}

fn output_mode() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = IoMode::Eval(EvalMode::Lazy);
    named_const_fn(
        "function.output_mode",
        input_mode,
        output_mode,
        fn_output_mode,
    )
}

fn fn_output_mode(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        generate_io_mode(&func.output_mode)
    })
}

fn is_primitive() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
    named_const_fn(
        "function.is_primitive",
        input_mode,
        output_mode,
        fn_is_primitive,
    )
}

fn fn_is_primitive(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let is_primitive = func.is_primitive();
        Val::Bool(Bool::new(is_primitive))
    })
}

fn is_extension() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
    named_const_fn(
        "function.is_extension",
        input_mode,
        output_mode,
        fn_is_extension,
    )
}

fn fn_is_extension(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let Some(is_extension) = func.primitive_is_extension() else {
            return Val::default();
        };
        Val::Bool(Bool::new(is_extension))
    })
}

fn id() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = symbol_value_mode();
    named_const_fn("function.id", input_mode, output_mode, fn_id)
}

fn fn_id(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let Some(id) = func.primitive_id() else {
            return Val::default();
        };
        Val::Symbol(id)
    })
}

fn body() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = IoMode::Eval(EvalMode::Lazy);
    named_const_fn("function.body", input_mode, output_mode, fn_body)
}

fn fn_body(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let Some(body) = func.composed_body() else {
            return Val::default();
        };
        body
    })
}

fn ctx() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
    named_const_fn("function.context", input_mode, output_mode, fn_ctx)
}

fn fn_ctx(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let Some(ctx) = func.composed_context() else {
            return Val::default();
        };
        Val::Ctx(CtxVal(Box::new(ctx)))
    })
}

fn input_name() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = symbol_value_mode();
    named_const_fn(
        "function.input_name",
        input_mode,
        output_mode,
        fn_input_name,
    )
}

fn fn_input_name(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let Some(name) = func.composed_input_name() else {
            return Val::default();
        };
        Val::Symbol(name)
    })
}

fn caller_name() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = symbol_value_mode();
    named_const_fn(
        "function.caller_name",
        input_mode,
        output_mode,
        fn_caller_name,
    )
}

fn fn_caller_name(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let Some(name) = func.composed_caller_name() else {
            return Val::default();
        };
        Val::Symbol(name)
    })
}
