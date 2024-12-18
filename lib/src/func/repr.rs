use crate::{
    Bit,
    Ctx,
    CtxVal,
    FuncVal,
    Map,
    Mode,
    PrimitiveMode,
    Symbol,
    Val,
    func::{
        Func,
        FuncImpl,
        FuncMode,
        cell::CellCompExt,
        comp::Composite,
        const1::ConstCompExt,
        free::FreeCompExt,
        mode::ModeFunc,
        mut1::MutCompExt,
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
    val::func::{
        cell::CellFuncVal,
        const1::ConstFuncVal,
        free::FreeFuncVal,
        mut1::MutFuncVal,
    },
};

pub(crate) const BODY_MODE: &str = "body_mode";
pub(crate) const BODY: &str = "body";
pub(crate) const PRELUDE: &str = "prelude";
pub(crate) const INPUT_NAME: &str = "input_name";
pub(crate) const CTX_NAME: &str = "context_name";
pub(crate) const ID: &str = "id";
pub(crate) const IS_EXTENSION: &str = "is_extension";
pub(crate) const CALL_MODE: &str = "call_mode";
pub(crate) const ABSTRACT_MODE: &str = "abstract_mode";
pub(crate) const ASK_MODE: &str = "ask_mode";
pub(crate) const CACHEABLE: &str = "cacheable";
pub(crate) const CTX_ACCESS: &str = "context_access";
pub(crate) const CELL: &str = "cell";

pub(crate) const DEFAULT_INPUT_NAME: &str = "i";
pub(crate) const DEFAULT_CTX_NAME: &str = "c";
pub(crate) const FREE: &str = "free";
pub(crate) const CONST: &str = "constant";
pub(crate) const MUTABLE: &str = "mutable";

pub(crate) fn parse_mode() -> Mode {
    let mut map = Map::default();
    map.insert(symbol(BODY_MODE), Mode::default());
    map.insert(symbol(BODY), form_mode());
    map.insert(symbol(PRELUDE), Mode::default());
    map.insert(symbol(INPUT_NAME), Mode::default());
    map.insert(symbol(CTX_NAME), Mode::default());
    map.insert(symbol(CTX_ACCESS), Mode::default());
    map.insert(symbol(CALL_MODE), Mode::default());
    map.insert(symbol(ABSTRACT_MODE), Mode::default());
    map.insert(symbol(ASK_MODE), Mode::default());
    map.insert(symbol(CACHEABLE), Mode::default());
    map.insert(symbol(CELL), Mode::default());
    map_mode(
        map,
        Mode::default(),
        Mode::default(),
        PrimitiveMode::default(),
    )
}

pub(crate) fn generate_mode() -> Mode {
    let mut map = Map::default();
    map.insert(symbol(BODY_MODE), Mode::default());
    map.insert(symbol(BODY), form_mode());
    map.insert(symbol(PRELUDE), Mode::default());
    map.insert(symbol(INPUT_NAME), Mode::default());
    map.insert(symbol(CTX_NAME), Mode::default());
    map.insert(symbol(CTX_ACCESS), Mode::default());
    map.insert(symbol(CALL_MODE), Mode::default());
    map.insert(symbol(ABSTRACT_MODE), Mode::default());
    map.insert(symbol(ASK_MODE), Mode::default());
    map.insert(symbol(CACHEABLE), Mode::default());
    map.insert(symbol(ID), Mode::default());
    map.insert(symbol(IS_EXTENSION), Mode::default());
    map.insert(symbol(CELL), Mode::default());
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
    let body_mode = match map_remove(&mut map, BODY_MODE) {
        Val::Unit(_) => Mode::default(),
        Val::Func(FuncVal::Mode(mode)) => mode.mode().clone(),
        _ => return None,
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
    let call = match map_remove(&mut map, CALL_MODE) {
        Val::Unit(_) => Mode::default(),
        Val::Func(FuncVal::Mode(call_mode)) => call_mode.mode().clone(),
        _ => return None,
    };
    let abstract1 = match map_remove(&mut map, ABSTRACT_MODE) {
        Val::Unit(_) => Mode::default(),
        Val::Func(FuncVal::Mode(abstract_mode)) => abstract_mode.mode().clone(),
        _ => return None,
    };
    let ask = match map_remove(&mut map, ASK_MODE) {
        Val::Unit(_) => Mode::default(),
        Val::Func(FuncVal::Mode(ask_mode)) => ask_mode.mode().clone(),
        _ => return None,
    };
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = match map_remove(&mut map, CACHEABLE) {
        Val::Unit(_) => false,
        Val::Bit(b) => b.bool(),
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
    let cell = match map_remove(&mut map, CELL) {
        Val::Unit(_) => false,
        Val::Bit(b) => b.bool(),
        _ => return None,
    };
    let func = if cell {
        let transformer = Composite {
            body_mode,
            body,
            prelude,
            input_name,
            ext: CellCompExt {},
        };
        let func = Func::new_composite(mode, cacheable, transformer);
        FuncVal::Cell(CellFuncVal::from(func))
    } else {
        match ctx_access {
            FREE => {
                let transformer = Composite {
                    body_mode,
                    body,
                    prelude,
                    input_name,
                    ext: FreeCompExt {},
                };
                let func = Func::new_composite(mode, cacheable, transformer);
                FuncVal::Free(FreeFuncVal::from(func))
            }
            CONST => {
                let transformer = Composite {
                    body_mode,
                    body,
                    prelude,
                    input_name,
                    ext: ConstCompExt { ctx_name },
                };
                let func = Func::new_composite(mode, cacheable, transformer);
                FuncVal::Const(ConstFuncVal::from(func))
            }
            MUTABLE => {
                let transformer = Composite {
                    body_mode,
                    body,
                    prelude,
                    input_name,
                    ext: MutCompExt { ctx_name },
                };
                let func = Func::new_composite(mode, cacheable, transformer);
                FuncVal::Mut(MutFuncVal::from(func))
            }
            _ => return None,
        }
    };
    Some(func)
}

pub(crate) fn generate_func(f: FuncVal) -> Val {
    match f {
        FuncVal::Mode(f) => generate(f.mode()),
        FuncVal::Cell(f) => generate_cell(f),
        FuncVal::Free(f) => generate_free(f),
        FuncVal::Const(f) => generate_const(f),
        FuncVal::Mut(f) => generate_mut(f),
    }
}

fn generate_cell(f: CellFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    let f = f.unwrap();
    match f.transformer {
        FuncImpl::Primitive(p) => {
            if p.is_extension {
                generate_extension(p.id, true, FREE, f.cacheable, f.mode, &mut repr);
            } else {
                repr.insert(symbol(ID), Val::Symbol(p.id));
            }
        }
        FuncImpl::Composite(c) => {
            generate_composite(
                true,
                FREE,
                f.cacheable,
                f.mode,
                c.body_mode,
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

fn generate_free(f: FreeFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    match &f.transformer {
        FuncImpl::Primitive(p) => {
            if p.is_extension {
                generate_extension(
                    p.id.clone(),
                    false,
                    FREE,
                    f.cacheable,
                    f.mode.clone(),
                    &mut repr,
                );
            } else {
                repr.insert(symbol(ID), Val::Symbol(p.id.clone()));
            }
        }
        FuncImpl::Composite(c) => generate_composite(
            false,
            FREE,
            f.cacheable,
            f.mode.clone(),
            c.body_mode.clone(),
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
                    f.mode.clone(),
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
            f.mode.clone(),
            c.body_mode.clone(),
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
                    f.mode.clone(),
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
            f.mode.clone(),
            c.body_mode.clone(),
            c.body.clone(),
            c.prelude.clone(),
            c.input_name.clone(),
            Some(c.ext.ctx_name.clone()),
            &mut repr,
        ),
    }
    Val::Map(repr.into())
}

#[allow(clippy::too_many_arguments)]
fn generate_extension(
    id: Symbol,
    cell: bool,
    access: &str,
    cacheable: bool,
    mode: FuncMode,
    repr: &mut Map<Val, Val>,
) {
    repr.insert(symbol(ID), Val::Symbol(id));
    repr.insert(symbol(IS_EXTENSION), Val::Bit(Bit::t()));
    generate_func_common(cell, access, cacheable, mode, repr);
}

#[allow(clippy::too_many_arguments)]
fn generate_composite(
    cell: bool,
    access: &str,
    cacheable: bool,
    mode: FuncMode,
    body_mode: Mode,
    body: Val,
    prelude: Ctx,
    input_name: Symbol,
    ctx_name: Option<Symbol>,
    repr: &mut Map<Val, Val>,
) {
    generate_func_common(cell, access, cacheable, mode, repr);
    if body_mode != Mode::default() {
        let mode = Val::Func(FuncVal::Mode(ModeFunc::new(body_mode).into()));
        repr.insert(symbol(BODY_MODE), mode);
    }
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
    cell: bool,
    access: &str,
    cacheable: bool,
    mode: FuncMode,
    repr: &mut Map<Val, Val>,
) {
    if cell {
        repr.insert(symbol(CELL), Val::Bit(Bit::t()));
    }
    if access != MUTABLE {
        repr.insert(symbol(CTX_ACCESS), symbol(access));
    }
    if cacheable {
        repr.insert(symbol(CACHEABLE), Val::Bit(Bit::new(true)));
    }
    if mode.call != Mode::default() {
        let call_mode = Val::Func(FuncVal::Mode(ModeFunc::new(mode.call).into()));
        repr.insert(symbol(CALL_MODE), call_mode);
    }
    if mode.abstract1 != Mode::default() {
        let abstract_mode = Val::Func(FuncVal::Mode(ModeFunc::new(mode.abstract1).into()));
        repr.insert(symbol(ABSTRACT_MODE), abstract_mode);
    }
    if mode.ask != Mode::default() {
        let ask_mode = Val::Func(FuncVal::Mode(ModeFunc::new(mode.ask).into()));
        repr.insert(symbol(ASK_MODE), ask_mode);
    }
}
