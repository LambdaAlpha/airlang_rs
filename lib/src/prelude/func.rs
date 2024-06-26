use crate::{
    bool::Bool,
    ctx::{
        constant::CtxForConstFn,
        Ctx,
        CtxMap,
        DefaultCtx,
    },
    func::{
        Composed,
        CtxConst,
        CtxConstInfo,
        CtxFree,
        CtxFreeInfo,
        CtxMutable,
        CtxMutableInfo,
        Func,
        FuncImpl,
        FuncTransformer,
    },
    map::Map,
    mode::{
        repr::{
            generate,
            parse,
        },
        Mode,
    },
    prelude::{
        form_mode,
        map_mode,
        named_const_fn,
        named_free_fn,
        Named,
        Prelude,
    },
    symbol::Symbol,
    transform::{
        Transform,
        EVAL,
    },
    utils::val::{
        map_remove,
        symbol,
    },
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
    pub(crate) ctx_access: Named<FuncVal>,
    pub(crate) input_mode: Named<FuncVal>,
    pub(crate) output_mode: Named<FuncVal>,
    pub(crate) is_primitive: Named<FuncVal>,
    pub(crate) is_extension: Named<FuncVal>,
    pub(crate) id: Named<FuncVal>,
    pub(crate) body: Named<FuncVal>,
    pub(crate) prelude: Named<FuncVal>,
    pub(crate) input_name: Named<FuncVal>,
    pub(crate) ctx_name: Named<FuncVal>,
}

impl Default for FuncPrelude {
    fn default() -> Self {
        FuncPrelude {
            new: new(),
            repr: repr(),
            ctx_access: ctx_access(),
            input_mode: input_mode(),
            output_mode: output_mode(),
            is_primitive: is_primitive(),
            is_extension: is_extension(),
            id: id(),
            body: body(),
            prelude: prelude(),
            input_name: input_name(),
            ctx_name: ctx_name(),
        }
    }
}

impl Prelude for FuncPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.new.put(m);
        self.repr.put(m);
        self.ctx_access.put(m);
        self.input_mode.put(m);
        self.output_mode.put(m);
        self.is_primitive.put(m);
        self.is_extension.put(m);
        self.id.put(m);
        self.body.put(m);
        self.prelude.put(m);
        self.input_name.put(m);
        self.ctx_name.put(m);
    }
}

const BODY: &str = "body";
const PRELUDE: &str = "prelude";
const INPUT_NAME: &str = "input_name";
const CTX_NAME: &str = "context_name";
const ID: &str = "id";
const IS_EXTENSION: &str = "is_extension";
const INPUT_MODE: &str = "input_mode";
const OUTPUT_MODE: &str = "output_mode";
const CTX_ACCESS: &str = "context_access";

const DEFAULT_INPUT_NAME: &str = "the_input";
const DEFAULT_CTX_NAME: &str = "the_context";
const FREE: &str = "free";
const CONST: &str = "constant";
const MUTABLE: &str = "mutable";

fn new() -> Named<FuncVal> {
    let mut map = Map::default();
    map.insert(symbol(BODY), form_mode());
    map.insert(symbol(PRELUDE), Mode::default());
    map.insert(symbol(INPUT_NAME), Mode::default());
    map.insert(symbol(CTX_NAME), Mode::default());
    map.insert(symbol(CTX_ACCESS), Mode::default());
    map.insert(symbol(INPUT_MODE), form_mode());
    map.insert(symbol(OUTPUT_MODE), form_mode());
    let input_mode = map_mode(map, Transform::default());
    let output_mode = Mode::default();
    named_free_fn("function", input_mode, output_mode, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let body = map_remove(&mut map, BODY);
    let prelude = match map_remove(&mut map, PRELUDE) {
        Val::Ctx(prelude) => prelude.into(),
        Val::Unit(_) => Ctx::default(),
        _ => return Val::default(),
    };
    let input_name = match map_remove(&mut map, INPUT_NAME) {
        Val::Symbol(name) => name,
        Val::Unit(_) => Symbol::from_str(DEFAULT_INPUT_NAME),
        _ => return Val::default(),
    };
    let input_mode = map.remove(&symbol(INPUT_MODE)).unwrap_or(symbol(EVAL));
    let Some(input_mode) = parse(input_mode) else {
        return Val::default();
    };
    let output_mode = map.remove(&symbol(OUTPUT_MODE)).unwrap_or(symbol(EVAL));
    let Some(output_mode) = parse(output_mode) else {
        return Val::default();
    };
    let ctx_name = match map_remove(&mut map, CTX_NAME) {
        Val::Symbol(name) => name,
        Val::Unit(_) => Symbol::from_str(DEFAULT_CTX_NAME),
        _ => return Val::default(),
    };
    let ctx_access = map_remove(&mut map, CTX_ACCESS);
    let ctx_access = match &ctx_access {
        Val::Symbol(s) => &**s,
        Val::Unit(_) => FREE,
        _ => return Val::default(),
    };
    let transformer = match ctx_access {
        FREE => FuncTransformer::Free(FuncImpl::Composed(Composed {
            body,
            prelude,
            input_name,
            ctx: CtxFreeInfo {},
        })),
        CONST => FuncTransformer::Const(FuncImpl::Composed(Composed {
            body,
            prelude,
            input_name,
            ctx: CtxConstInfo { name: ctx_name },
        })),
        MUTABLE => FuncTransformer::Mutable(FuncImpl::Composed(Composed {
            body,
            prelude,
            input_name,
            ctx: CtxMutableInfo { name: ctx_name },
        })),
        _ => return Val::default(),
    };
    let func = Func::new(input_mode, output_mode, transformer);
    Val::Func(FuncVal::from(func))
}

fn repr() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let mut map = Map::default();
    map.insert(symbol(BODY), form_mode());
    map.insert(symbol(PRELUDE), Mode::default());
    map.insert(symbol(INPUT_NAME), Mode::default());
    map.insert(symbol(CTX_NAME), Mode::default());
    map.insert(symbol(CTX_ACCESS), Mode::default());
    map.insert(symbol(INPUT_MODE), form_mode());
    map.insert(symbol(OUTPUT_MODE), form_mode());
    map.insert(symbol(ID), Mode::default());
    map.insert(symbol(IS_EXTENSION), Mode::default());
    let output_mode = map_mode(map, Transform::default());
    named_free_fn("function.represent", input_mode, output_mode, fn_repr)
}

fn fn_repr(input: Val) -> Val {
    let Val::Func(func) = input else {
        return Val::default();
    };
    let mut repr = MapVal::from(Map::<Val, Val>::default());

    if func.input_mode != Mode::default() {
        let input_mode = generate(&func.input_mode);
        repr.insert(symbol(INPUT_MODE), input_mode);
    }
    if func.output_mode != Mode::default() {
        let output_mode = generate(&func.output_mode);
        repr.insert(symbol(OUTPUT_MODE), output_mode);
    }

    match &func.transformer {
        FuncTransformer::Free(t) => match t {
            CtxFree::Primitive(p) => {
                repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
                if p.is_extension() {
                    repr.insert(symbol(IS_EXTENSION), Val::Bool(Bool::t()));
                }
            }
            CtxFree::Composed(c) => {
                repr.insert(symbol(BODY), c.body.clone());
                if c.prelude != Ctx::default() {
                    repr.insert(symbol(PRELUDE), Val::Ctx(CtxVal::from(c.prelude.clone())));
                }
                if &*c.input_name != DEFAULT_INPUT_NAME {
                    repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                }
            }
        },
        FuncTransformer::Const(t) => {
            repr.insert(symbol(CTX_ACCESS), symbol(CONST));
            match t {
                CtxConst::Primitive(p) => {
                    repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
                    if p.is_extension() {
                        repr.insert(symbol(IS_EXTENSION), Val::Bool(Bool::t()));
                    }
                }
                CtxConst::Composed(c) => {
                    repr.insert(symbol(BODY), c.body.clone());
                    if c.prelude != Ctx::default() {
                        repr.insert(symbol(PRELUDE), Val::Ctx(CtxVal::from(c.prelude.clone())));
                    }
                    if &*c.input_name != DEFAULT_INPUT_NAME {
                        repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                    }
                    if &*c.ctx.name != DEFAULT_CTX_NAME {
                        repr.insert(symbol(CTX_NAME), Val::Symbol(c.ctx.name.clone()));
                    }
                }
            }
        }
        FuncTransformer::Mutable(t) => {
            repr.insert(symbol(CTX_ACCESS), symbol(MUTABLE));
            match t {
                CtxMutable::Primitive(p) => {
                    repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
                    if p.is_extension() {
                        repr.insert(symbol(IS_EXTENSION), Val::Bool(Bool::t()));
                    }
                }
                CtxMutable::Composed(c) => {
                    repr.insert(symbol(BODY), c.body.clone());
                    if c.prelude != Ctx::default() {
                        repr.insert(symbol(PRELUDE), Val::Ctx(CtxVal::from(c.prelude.clone())));
                    }
                    if &*c.input_name != DEFAULT_INPUT_NAME {
                        repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                    }
                    if &*c.ctx.name != DEFAULT_CTX_NAME {
                        repr.insert(symbol(CTX_NAME), Val::Symbol(c.ctx.name.clone()));
                    }
                }
            }
        }
    }
    Val::Map(repr)
}

fn ctx_access() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn(
        "function.context_access",
        input_mode,
        output_mode,
        fn_ctx_access,
    )
}

fn fn_ctx_access(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let access = match &func.transformer {
            FuncTransformer::Free(_) => FREE,
            FuncTransformer::Const(_) => CONST,
            FuncTransformer::Mutable(_) => MUTABLE,
        };
        Val::Symbol(Symbol::from_str(access))
    })
}

fn input_mode() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn(
        "function.input_mode",
        input_mode,
        output_mode,
        fn_input_mode,
    )
}

fn fn_input_mode(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        generate(&func.input_mode)
    })
}

fn output_mode() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn(
        "function.output_mode",
        input_mode,
        output_mode,
        fn_output_mode,
    )
}

fn fn_output_mode(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        generate(&func.output_mode)
    })
}

fn is_primitive() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn(
        "function.is_primitive",
        input_mode,
        output_mode,
        fn_is_primitive,
    )
}

fn fn_is_primitive(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let is_primitive = func.is_primitive();
        Val::Bool(Bool::new(is_primitive))
    })
}

fn is_extension() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn(
        "function.is_extension",
        input_mode,
        output_mode,
        fn_is_extension,
    )
}

fn fn_is_extension(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(is_extension) = func.primitive_is_extension() else {
            return Val::default();
        };
        Val::Bool(Bool::new(is_extension))
    })
}

fn id() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("function.id", input_mode, output_mode, fn_id)
}

fn fn_id(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(id) = func.primitive_id() else {
            return Val::default();
        };
        Val::Symbol(id)
    })
}

fn body() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("function.body", input_mode, output_mode, fn_body)
}

fn fn_body(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(body) = func.composed_body() else {
            return Val::default();
        };
        body
    })
}

fn prelude() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("function.prelude", input_mode, output_mode, fn_prelude)
}

fn fn_prelude(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(prelude) = func.composed_prelude() else {
            return Val::default();
        };
        Val::Ctx(CtxVal::from(prelude))
    })
}

fn input_name() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn(
        "function.input_name",
        input_mode,
        output_mode,
        fn_input_name,
    )
}

fn fn_input_name(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(name) = func.composed_input_name() else {
            return Val::default();
        };
        Val::Symbol(name)
    })
}

fn ctx_name() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn(
        "function.context_name",
        input_mode,
        output_mode,
        fn_ctx_name,
    )
}

fn fn_ctx_name(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(name) = func.composed_ctx_name() else {
            return Val::default();
        };
        Val::Symbol(name)
    })
}
