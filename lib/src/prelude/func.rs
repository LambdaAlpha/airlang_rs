use crate::{
    bool::Bool,
    ctx::{
        const1::ConstFnCtx,
        default::DefaultCtx,
        Ctx,
        CtxValue,
    },
    func::{
        const1::ConstCompositeExt,
        free::FreeCompositeExt,
        mut1::MutCompositeExt,
        static1::StaticCompositeExt,
        Composite,
        Func,
        FuncImpl,
    },
    map::Map,
    mode::{
        basic::BasicMode,
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
        named_static_fn,
        Named,
        Prelude,
    },
    symbol::Symbol,
    utils::val::{
        map_remove,
        symbol,
    },
    val::{
        ctx::CtxVal,
        func::FuncVal,
        Val,
    },
    ConstFuncVal,
    FreeFuncVal,
    MutFuncVal,
    StaticFuncVal,
};

#[derive(Clone)]
pub(crate) struct FuncPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) repr: Named<FuncVal>,
    pub(crate) ctx_access: Named<FuncVal>,
    pub(crate) call_mode: Named<FuncVal>,
    pub(crate) ask_mode: Named<FuncVal>,
    pub(crate) is_cacheable: Named<FuncVal>,
    pub(crate) is_primitive: Named<FuncVal>,
    pub(crate) is_extension: Named<FuncVal>,
    pub(crate) is_static: Named<FuncVal>,
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
            call_mode: call_mode(),
            ask_mode: ask_mode(),
            is_cacheable: is_cacheable(),
            is_primitive: is_primitive(),
            is_extension: is_extension(),
            is_static: is_static(),
            id: id(),
            body: body(),
            prelude: prelude(),
            input_name: input_name(),
            ctx_name: ctx_name(),
        }
    }
}

impl Prelude for FuncPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.repr.put(m);
        self.ctx_access.put(m);
        self.call_mode.put(m);
        self.ask_mode.put(m);
        self.is_cacheable.put(m);
        self.is_primitive.put(m);
        self.is_extension.put(m);
        self.is_static.put(m);
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
const CALL_MODE: &str = "call_mode";
const ASK_MODE: &str = "ask_mode";
const CACHEABLE: &str = "cacheable";
const CTX_ACCESS: &str = "context_access";
const STATIC: &str = "static";

const DEFAULT_INPUT_NAME: &str = "i";
const DEFAULT_CTX_NAME: &str = "c";
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
    map.insert(symbol(CALL_MODE), form_mode());
    map.insert(symbol(ASK_MODE), form_mode());
    map.insert(symbol(CACHEABLE), Mode::default());
    map.insert(symbol(STATIC), Mode::default());
    let call_mode = map_mode(map, Mode::default(), Mode::default(), BasicMode::default());
    let ask_mode = Mode::default();
    named_static_fn("function", call_mode, ask_mode, true, fn_new)
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
    let call_mode = map_remove(&mut map, CALL_MODE);
    let Some(call_mode) = parse(call_mode) else {
        return Val::default();
    };
    let ask_mode = map_remove(&mut map, ASK_MODE);
    let Some(ask_mode) = parse(ask_mode) else {
        return Val::default();
    };
    let cacheable = match map_remove(&mut map, CACHEABLE) {
        Val::Unit(_) => false,
        Val::Bool(b) => b.bool(),
        _ => return Val::default(),
    };
    let ctx_name = match map_remove(&mut map, CTX_NAME) {
        Val::Symbol(name) => name,
        Val::Unit(_) => Symbol::from_str(DEFAULT_CTX_NAME),
        _ => return Val::default(),
    };
    let ctx_access = map_remove(&mut map, CTX_ACCESS);
    let ctx_access = match &ctx_access {
        Val::Symbol(s) => &**s,
        Val::Unit(_) => MUTABLE,
        _ => return Val::default(),
    };
    let static1 = match map_remove(&mut map, STATIC) {
        Val::Unit(_) => false,
        Val::Bool(b) => b.bool(),
        _ => return Val::default(),
    };
    let func = match ctx_access {
        FREE => {
            if static1 {
                let transformer = Composite {
                    body,
                    prelude,
                    input_name,
                    ext: StaticCompositeExt {},
                };
                let func = Func::new_composite(call_mode, ask_mode, cacheable, transformer);
                FuncVal::Static(StaticFuncVal::from(func))
            } else {
                let transformer = Composite {
                    body,
                    prelude,
                    input_name,
                    ext: FreeCompositeExt {},
                };
                let func = Func::new_composite(call_mode, ask_mode, cacheable, transformer);
                FuncVal::Free(FreeFuncVal::from(func))
            }
        }
        CONST => {
            let transformer = Composite {
                body,
                prelude,
                input_name,
                ext: ConstCompositeExt { ctx_name },
            };
            let func = Func::new_composite(call_mode, ask_mode, cacheable, transformer);
            FuncVal::Const(ConstFuncVal::from(func))
        }
        MUTABLE => {
            let transformer = Composite {
                body,
                prelude,
                input_name,
                ext: MutCompositeExt { ctx_name },
            };
            let func = Func::new_composite(call_mode, ask_mode, cacheable, transformer);
            FuncVal::Mut(MutFuncVal::from(func))
        }
        _ => return Val::default(),
    };
    Val::Func(func)
}

fn repr() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let mut map = Map::default();
    map.insert(symbol(BODY), form_mode());
    map.insert(symbol(PRELUDE), Mode::default());
    map.insert(symbol(INPUT_NAME), Mode::default());
    map.insert(symbol(CTX_NAME), Mode::default());
    map.insert(symbol(CTX_ACCESS), Mode::default());
    map.insert(symbol(CALL_MODE), form_mode());
    map.insert(symbol(ASK_MODE), form_mode());
    map.insert(symbol(CACHEABLE), Mode::default());
    map.insert(symbol(ID), Mode::default());
    map.insert(symbol(IS_EXTENSION), Mode::default());
    map.insert(symbol(STATIC), Mode::default());
    let ask_mode = map_mode(map, Mode::default(), Mode::default(), BasicMode::default());
    named_static_fn("function.represent", call_mode, ask_mode, true, fn_repr)
}

fn fn_repr(input: Val) -> Val {
    let Val::Func(func) = input else {
        return Val::default();
    };
    let mut repr = Map::<Val, Val>::default();

    match func {
        FuncVal::Free(f) => {
            let f = f.unwrap();
            match f.transformer {
                FuncImpl::Primitive(p) => {
                    repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
                    if p.is_extension() {
                        repr.insert(symbol(CTX_ACCESS), symbol(FREE));
                        repr.insert(symbol(IS_EXTENSION), Val::Bool(Bool::t()));
                        generate_cacheable(f.cacheable, &mut repr);
                        generate_mode(&f.call_mode, &f.ask_mode, &mut repr);
                    }
                }
                FuncImpl::Composite(c) => {
                    repr.insert(symbol(CTX_ACCESS), symbol(FREE));
                    if c.body != Val::default() {
                        repr.insert(symbol(BODY), c.body);
                    }
                    if c.prelude != Ctx::default() {
                        repr.insert(symbol(PRELUDE), Val::Ctx(CtxVal::from(c.prelude)));
                    }
                    if &*c.input_name != DEFAULT_INPUT_NAME {
                        repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name));
                    }
                    generate_cacheable(f.cacheable, &mut repr);
                    generate_mode(&f.call_mode, &f.ask_mode, &mut repr);
                }
            }
        }
        FuncVal::Static(f) => match &f.transformer {
            FuncImpl::Primitive(p) => {
                repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
                if p.is_extension() {
                    repr.insert(symbol(STATIC), Val::Bool(Bool::t()));
                    repr.insert(symbol(CTX_ACCESS), symbol(FREE));
                    repr.insert(symbol(IS_EXTENSION), Val::Bool(Bool::t()));
                    generate_cacheable(f.cacheable(), &mut repr);
                    generate_mode(f.call_mode(), f.ask_mode(), &mut repr);
                }
            }
            FuncImpl::Composite(c) => {
                repr.insert(symbol(STATIC), Val::Bool(Bool::t()));
                repr.insert(symbol(CTX_ACCESS), symbol(FREE));
                if c.body != Val::default() {
                    repr.insert(symbol(BODY), c.body.clone());
                }
                if c.prelude != Ctx::default() {
                    repr.insert(symbol(PRELUDE), Val::Ctx(CtxVal::from(c.prelude.clone())));
                }
                if &*c.input_name != DEFAULT_INPUT_NAME {
                    repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                }
                generate_cacheable(f.cacheable(), &mut repr);
                generate_mode(f.call_mode(), f.ask_mode(), &mut repr);
            }
        },
        FuncVal::Const(f) => match &f.transformer {
            FuncImpl::Primitive(p) => {
                repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
                if p.is_extension() {
                    repr.insert(symbol(CTX_ACCESS), symbol(CONST));
                    repr.insert(symbol(IS_EXTENSION), Val::Bool(Bool::t()));
                    generate_cacheable(f.cacheable(), &mut repr);
                    generate_mode(f.call_mode(), f.ask_mode(), &mut repr);
                }
            }
            FuncImpl::Composite(c) => {
                repr.insert(symbol(CTX_ACCESS), symbol(CONST));
                if c.body != Val::default() {
                    repr.insert(symbol(BODY), c.body.clone());
                }
                if c.prelude != Ctx::default() {
                    repr.insert(symbol(PRELUDE), Val::Ctx(CtxVal::from(c.prelude.clone())));
                }
                if &*c.input_name != DEFAULT_INPUT_NAME {
                    repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                }
                if &*c.ext.ctx_name != DEFAULT_CTX_NAME {
                    repr.insert(symbol(CTX_NAME), Val::Symbol(c.ext.ctx_name.clone()));
                }
                generate_cacheable(f.cacheable(), &mut repr);
                generate_mode(f.call_mode(), f.ask_mode(), &mut repr);
            }
        },
        FuncVal::Mut(f) => match &f.transformer {
            FuncImpl::Primitive(p) => {
                repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
                if p.is_extension() {
                    repr.insert(symbol(IS_EXTENSION), Val::Bool(Bool::t()));
                    generate_cacheable(f.cacheable(), &mut repr);
                    generate_mode(f.call_mode(), f.ask_mode(), &mut repr);
                }
            }
            FuncImpl::Composite(c) => {
                if c.body != Val::default() {
                    repr.insert(symbol(BODY), c.body.clone());
                }
                if c.prelude != Ctx::default() {
                    repr.insert(symbol(PRELUDE), Val::Ctx(CtxVal::from(c.prelude.clone())));
                }
                if &*c.input_name != DEFAULT_INPUT_NAME {
                    repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                }
                if &*c.ext.ctx_name != DEFAULT_CTX_NAME {
                    repr.insert(symbol(CTX_NAME), Val::Symbol(c.ext.ctx_name.clone()));
                }
                generate_cacheable(f.cacheable(), &mut repr);
                generate_mode(f.call_mode(), f.ask_mode(), &mut repr);
            }
        },
    }
    Val::Map(repr.into())
}

fn generate_mode(call_mode: &Mode, ask_mode: &Mode, repr: &mut Map<Val, Val>) {
    if *call_mode != Mode::default() {
        let call_mode = generate(call_mode);
        repr.insert(symbol(CALL_MODE), call_mode);
    }
    if *ask_mode != Mode::default() {
        let ask_mode = generate(ask_mode);
        repr.insert(symbol(ASK_MODE), ask_mode);
    }
}

fn generate_cacheable(cacheable: bool, repr: &mut Map<Val, Val>) {
    if cacheable {
        repr.insert(symbol(CACHEABLE), Val::Bool(Bool::new(cacheable)));
    }
}

fn ctx_access() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.context_access",
        call_mode,
        ask_mode,
        true,
        fn_ctx_access,
    )
}

fn fn_ctx_access(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let access = match func {
            FuncVal::Free(_) => FREE,
            FuncVal::Static(_) => FREE,
            FuncVal::Const(_) => CONST,
            FuncVal::Mut(_) => MUTABLE,
        };
        Val::Symbol(Symbol::from_str(access))
    })
}

fn call_mode() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.call_mode",
        call_mode,
        ask_mode,
        true,
        fn_call_mode,
    )
}

fn fn_call_mode(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        generate(func.call_mode())
    })
}

fn ask_mode() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("function.ask_mode", call_mode, ask_mode, true, fn_ask_mode)
}

fn fn_ask_mode(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        generate(func.ask_mode())
    })
}

fn is_cacheable() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.is_cacheable",
        call_mode,
        ask_mode,
        true,
        fn_is_cacheable,
    )
}

fn fn_is_cacheable(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let cacheable = func.cacheable();
        Val::Bool(Bool::new(cacheable))
    })
}

fn is_primitive() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.is_primitive",
        call_mode,
        ask_mode,
        true,
        fn_is_primitive,
    )
}

fn fn_is_primitive(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let is_primitive = func.is_primitive();
        Val::Bool(Bool::new(is_primitive))
    })
}

fn is_extension() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.is_extension",
        call_mode,
        ask_mode,
        true,
        fn_is_extension,
    )
}

fn fn_is_extension(ctx: ConstFnCtx, input: Val) -> Val {
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

fn is_static() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.is_static",
        call_mode,
        ask_mode,
        true,
        fn_is_static,
    )
}

fn fn_is_static(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        Val::Bool(Bool::new(matches!(func, FuncVal::Static(_))))
    })
}

fn id() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("function.id", call_mode, ask_mode, true, fn_id)
}

fn fn_id(ctx: ConstFnCtx, input: Val) -> Val {
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
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("function.body", call_mode, ask_mode, true, fn_body)
}

fn fn_body(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(body) = func.composite_body() else {
            return Val::default();
        };
        body.clone()
    })
}

fn prelude() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("function.prelude", call_mode, ask_mode, true, fn_prelude)
}

fn fn_prelude(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(prelude) = func.composite_prelude() else {
            return Val::default();
        };
        Val::Ctx(CtxVal::from(prelude.clone()))
    })
}

fn input_name() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.input_name",
        call_mode,
        ask_mode,
        true,
        fn_input_name,
    )
}

fn fn_input_name(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(name) = func.composite_input_name() else {
            return Val::default();
        };
        Val::Symbol(name)
    })
}

fn ctx_name() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.context_name",
        call_mode,
        ask_mode,
        true,
        fn_ctx_name,
    )
}

fn fn_ctx_name(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(name) = func.composite_ctx_name() else {
            return Val::default();
        };
        Val::Symbol(name)
    })
}
