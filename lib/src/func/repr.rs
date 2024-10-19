use crate::{
    Bool,
    ConstFuncVal,
    Ctx,
    CtxVal,
    FreeFuncVal,
    FuncVal,
    Map,
    Mode,
    MutFuncVal,
    PrimitiveMode,
    StaticFuncVal,
    Symbol,
    Val,
    func::{
        Composite,
        Func,
        FuncImpl,
        const1::ConstCompositeExt,
        free::FreeCompositeExt,
        mode::ModeFunc,
        mut1::MutCompositeExt,
        static1::StaticCompositeExt,
    },
    mode::repr::generate,
    prelude::{
        form_mode,
        map_mode,
    },
    utils::val::{
        map_remove,
        symbol,
    },
};

pub(crate) const BODY: &str = "body";
pub(crate) const PRELUDE: &str = "prelude";
pub(crate) const INPUT_NAME: &str = "input_name";
pub(crate) const CTX_NAME: &str = "context_name";
pub(crate) const ID: &str = "id";
pub(crate) const IS_EXTENSION: &str = "is_extension";
pub(crate) const CALL_MODE: &str = "call_mode";
pub(crate) const ASK_MODE: &str = "ask_mode";
pub(crate) const CACHEABLE: &str = "cacheable";
pub(crate) const CTX_ACCESS: &str = "context_access";
pub(crate) const STATIC: &str = "static";

pub(crate) const DEFAULT_INPUT_NAME: &str = "i";
pub(crate) const DEFAULT_CTX_NAME: &str = "c";
pub(crate) const FREE: &str = "free";
pub(crate) const CONST: &str = "constant";
pub(crate) const MUTABLE: &str = "mutable";

pub(crate) fn parse_mode() -> Mode {
    let mut map = Map::default();
    map.insert(symbol(BODY), form_mode());
    map.insert(symbol(PRELUDE), Mode::default());
    map.insert(symbol(INPUT_NAME), Mode::default());
    map.insert(symbol(CTX_NAME), Mode::default());
    map.insert(symbol(CTX_ACCESS), Mode::default());
    map.insert(symbol(CALL_MODE), Mode::default());
    map.insert(symbol(ASK_MODE), Mode::default());
    map.insert(symbol(CACHEABLE), Mode::default());
    map.insert(symbol(STATIC), Mode::default());
    map_mode(
        map,
        Mode::default(),
        Mode::default(),
        PrimitiveMode::default(),
    )
}

pub(crate) fn generate_mode() -> Mode {
    let mut map = Map::default();
    map.insert(symbol(BODY), form_mode());
    map.insert(symbol(PRELUDE), Mode::default());
    map.insert(symbol(INPUT_NAME), Mode::default());
    map.insert(symbol(CTX_NAME), Mode::default());
    map.insert(symbol(CTX_ACCESS), Mode::default());
    map.insert(symbol(CALL_MODE), Mode::default());
    map.insert(symbol(ASK_MODE), Mode::default());
    map.insert(symbol(CACHEABLE), Mode::default());
    map.insert(symbol(ID), Mode::default());
    map.insert(symbol(IS_EXTENSION), Mode::default());
    map.insert(symbol(STATIC), Mode::default());
    map_mode(
        map,
        Mode::default(),
        Mode::default(),
        PrimitiveMode::default(),
    )
}

pub(crate) fn parse_func(input: Val) -> Option<FuncVal> {
    let Val::Map(mut map) = input else {
        return None;
    };
    let body = map_remove(&mut map, BODY);
    let prelude = match map_remove(&mut map, PRELUDE) {
        Val::Ctx(prelude) => prelude.into(),
        Val::Unit(_) => Ctx::default(),
        _ => return None,
    };
    let input_name = match map_remove(&mut map, INPUT_NAME) {
        Val::Symbol(name) => name,
        Val::Unit(_) => Symbol::from_str(DEFAULT_INPUT_NAME),
        _ => return None,
    };
    let call_mode = match map_remove(&mut map, CALL_MODE) {
        Val::Unit(_) => Mode::default(),
        Val::Func(FuncVal::Mode(call_mode)) => call_mode.mode().clone(),
        _ => return None,
    };
    let ask_mode = match map_remove(&mut map, ASK_MODE) {
        Val::Unit(_) => Mode::default(),
        Val::Func(FuncVal::Mode(ask_mode)) => ask_mode.mode().clone(),
        _ => return None,
    };
    let cacheable = match map_remove(&mut map, CACHEABLE) {
        Val::Unit(_) => false,
        Val::Bool(b) => b.bool(),
        _ => return None,
    };
    let ctx_name = match map_remove(&mut map, CTX_NAME) {
        Val::Symbol(name) => name,
        Val::Unit(_) => Symbol::from_str(DEFAULT_CTX_NAME),
        _ => return None,
    };
    let ctx_access = map_remove(&mut map, CTX_ACCESS);
    let ctx_access = match &ctx_access {
        Val::Symbol(s) => &**s,
        Val::Unit(_) => MUTABLE,
        _ => return None,
    };
    let static1 = match map_remove(&mut map, STATIC) {
        Val::Unit(_) => false,
        Val::Bool(b) => b.bool(),
        _ => return None,
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
        _ => return None,
    };
    Some(func)
}

pub(crate) fn generate_func(f: FuncVal) -> Val {
    match f {
        FuncVal::Mode(f) => generate(f.mode()),
        FuncVal::Free(f) => generate_free(f),
        FuncVal::Static(f) => generate_static(f),
        FuncVal::Const(f) => generate_const(f),
        FuncVal::Mut(f) => generate_mut(f),
    }
}

fn generate_free(f: FreeFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    let f = f.unwrap();
    match f.transformer {
        FuncImpl::Primitive(p) => {
            if p.is_extension {
                generate_extension(
                    p.id,
                    false,
                    FREE,
                    f.cacheable,
                    f.call_mode,
                    f.ask_mode,
                    &mut repr,
                );
            } else {
                repr.insert(symbol(ID), Val::Symbol(p.id));
            }
        }
        FuncImpl::Composite(c) => {
            generate_composite(
                false,
                FREE,
                f.cacheable,
                f.call_mode,
                f.ask_mode,
                c.body,
                c.prelude,
                c.input_name,
                None,
                &mut repr,
            );
        }
    }
    Val::Map(repr.into())
}

fn generate_static(f: StaticFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    match &f.transformer {
        FuncImpl::Primitive(p) => {
            if p.is_extension {
                generate_extension(
                    p.id.clone(),
                    true,
                    FREE,
                    f.cacheable,
                    f.call_mode.clone(),
                    f.ask_mode.clone(),
                    &mut repr,
                );
            } else {
                repr.insert(symbol(ID), Val::Symbol(p.id.clone()));
            }
        }
        FuncImpl::Composite(c) => generate_composite(
            true,
            FREE,
            f.cacheable,
            f.call_mode.clone(),
            f.ask_mode.clone(),
            c.body.clone(),
            c.prelude.clone(),
            c.input_name.clone(),
            None,
            &mut repr,
        ),
    }
    Val::Map(repr.into())
}

fn generate_const(f: ConstFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    match &f.transformer {
        FuncImpl::Primitive(p) => {
            if p.is_extension {
                generate_extension(
                    p.id.clone(),
                    false,
                    CONST,
                    f.cacheable,
                    f.call_mode.clone(),
                    f.ask_mode.clone(),
                    &mut repr,
                );
            } else {
                repr.insert(symbol(ID), Val::Symbol(p.id.clone()));
            }
        }
        FuncImpl::Composite(c) => generate_composite(
            false,
            CONST,
            f.cacheable,
            f.call_mode.clone(),
            f.ask_mode.clone(),
            c.body.clone(),
            c.prelude.clone(),
            c.input_name.clone(),
            Some(c.ext.ctx_name.clone()),
            &mut repr,
        ),
    }
    Val::Map(repr.into())
}

fn generate_mut(f: MutFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    match &f.transformer {
        FuncImpl::Primitive(p) => {
            if p.is_extension {
                generate_extension(
                    p.id.clone(),
                    false,
                    MUTABLE,
                    f.cacheable,
                    f.call_mode.clone(),
                    f.ask_mode.clone(),
                    &mut repr,
                );
            } else {
                repr.insert(symbol(ID), Val::Symbol(p.id.clone()));
            }
        }
        FuncImpl::Composite(c) => generate_composite(
            false,
            MUTABLE,
            f.cacheable,
            f.call_mode.clone(),
            f.ask_mode.clone(),
            c.body.clone(),
            c.prelude.clone(),
            c.input_name.clone(),
            Some(c.ext.ctx_name.clone()),
            &mut repr,
        ),
    }
    Val::Map(repr.into())
}

fn generate_extension(
    id: Symbol,
    static1: bool,
    access: &str,
    cacheable: bool,
    call_mode: Mode,
    ask_mode: Mode,
    repr: &mut Map<Val, Val>,
) {
    repr.insert(symbol(ID), Val::Symbol(id));
    repr.insert(symbol(IS_EXTENSION), Val::Bool(Bool::t()));
    generate_func_common(static1, access, cacheable, call_mode, ask_mode, repr);
}

#[allow(clippy::too_many_arguments)]
fn generate_composite(
    static1: bool,
    access: &str,
    cacheable: bool,
    call_mode: Mode,
    ask_mode: Mode,
    body: Val,
    prelude: Ctx,
    input_name: Symbol,
    ctx_name: Option<Symbol>,
    repr: &mut Map<Val, Val>,
) {
    generate_func_common(static1, access, cacheable, call_mode, ask_mode, repr);
    if body != Val::default() {
        repr.insert(symbol(BODY), body);
    }
    if prelude != Ctx::default() {
        repr.insert(symbol(PRELUDE), Val::Ctx(CtxVal::from(prelude)));
    }
    if &*input_name != DEFAULT_INPUT_NAME {
        repr.insert(symbol(INPUT_NAME), Val::Symbol(input_name));
    }
    if let Some(ctx_name) = ctx_name {
        if &*ctx_name != DEFAULT_CTX_NAME {
            repr.insert(symbol(CTX_NAME), Val::Symbol(ctx_name));
        }
    }
}

fn generate_func_common(
    static1: bool,
    access: &str,
    cacheable: bool,
    call_mode: Mode,
    ask_mode: Mode,
    repr: &mut Map<Val, Val>,
) {
    if static1 {
        repr.insert(symbol(STATIC), Val::Bool(Bool::t()));
    }
    if access != MUTABLE {
        repr.insert(symbol(CTX_ACCESS), symbol(access));
    }
    if cacheable {
        repr.insert(symbol(CACHEABLE), Val::Bool(Bool::new(true)));
    }
    if call_mode != Mode::default() {
        let call_mode = Val::Func(FuncVal::Mode(ModeFunc::new(call_mode).into()));
        repr.insert(symbol(CALL_MODE), call_mode);
    }
    if ask_mode != Mode::default() {
        let ask_mode = Val::Func(FuncVal::Mode(ModeFunc::new(ask_mode).into()));
        repr.insert(symbol(ASK_MODE), ask_mode);
    }
}
