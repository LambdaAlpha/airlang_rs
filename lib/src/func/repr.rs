use crate::{
    Bit,
    ConstStaticCompFunc,
    ConstStaticCompFuncVal,
    ConstStaticPrimFuncVal,
    Ctx,
    CtxVal,
    FreeCellCompFunc,
    FreeCellCompFuncVal,
    FreeCellPrimFunc,
    FreeCellPrimFuncVal,
    FreeStaticCompFunc,
    FreeStaticCompFuncVal,
    FreeStaticPrimFuncVal,
    FuncVal,
    Map,
    Mode,
    ModeFunc,
    MutStaticCompFunc,
    MutStaticCompFuncVal,
    MutStaticPrimFuncVal,
    PrimitiveMode,
    Symbol,
    Val,
    func::{
        FuncMode,
        comp::Composite,
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
        Val::Func(FuncVal::Mode(mode)) => mode.self_mode().clone(),
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
        Val::Func(FuncVal::Mode(call_mode)) => call_mode.self_mode().clone(),
        _ => return None,
    };
    let abstract1 = match map_remove(&mut map, ABSTRACT_MODE) {
        Val::Unit(_) => Mode::default(),
        Val::Func(FuncVal::Mode(abstract_mode)) => abstract_mode.self_mode().clone(),
        _ => return None,
    };
    let ask = match map_remove(&mut map, ASK_MODE) {
        Val::Unit(_) => Mode::default(),
        Val::Func(FuncVal::Mode(ask_mode)) => ask_mode.self_mode().clone(),
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
    let comp = Composite {
        body_mode,
        body,
        prelude,
        input_name,
    };
    let func = if cell {
        let func = FreeCellCompFunc::new(comp, mode, cacheable);
        FuncVal::FreeCellComp(FreeCellCompFuncVal::from(func))
    } else {
        match ctx_access {
            FREE => {
                let func = FreeStaticCompFunc::new(comp, mode, cacheable);
                FuncVal::FreeStaticComp(FreeStaticCompFuncVal::from(func))
            }
            CONST => {
                let func = ConstStaticCompFunc::new(comp, mode, cacheable, ctx_name);
                FuncVal::ConstStaticComp(ConstStaticCompFuncVal::from(func))
            }
            MUTABLE => {
                let func = MutStaticCompFunc::new(comp, mode, cacheable, ctx_name);
                FuncVal::MutStaticComp(MutStaticCompFuncVal::from(func))
            }
            _ => return None,
        }
    };
    Some(func)
}

pub(crate) fn generate_func(f: FuncVal) -> Val {
    match f {
        FuncVal::Mode(f) => generate(f.self_mode()),
        FuncVal::FreeCellPrim(f) => generate_free_cell_prim(f),
        FuncVal::FreeCellComp(f) => generate_free_cell_comp(f),
        FuncVal::FreeStaticPrim(f) => generate_free_prim(f),
        FuncVal::FreeStaticComp(f) => generate_free_comp(f),
        FuncVal::ConstStaticPrim(f) => generate_const_prim(f),
        FuncVal::ConstStaticComp(f) => generate_const_comp(f),
        FuncVal::MutStaticPrim(f) => generate_mut_prim(f),
        FuncVal::MutStaticComp(f) => generate_mut_comp(f),
    }
}

fn generate_free_cell_prim(f: FreeCellPrimFuncVal) -> Val {
    let f = FreeCellPrimFunc::from(f);
    let mut repr = Map::<Val, Val>::default();
    if f.prim.is_extension {
        generate_extension(f.prim.id, true, FREE, f.cacheable, f.mode, &mut repr);
    } else {
        repr.insert(symbol(ID), Val::Symbol(f.prim.id));
    }
    Val::Map(repr.into())
}

fn generate_free_cell_comp(f: FreeCellCompFuncVal) -> Val {
    let f = FreeCellCompFunc::from(f);
    let mut repr = Map::<Val, Val>::default();
    generate_composite(true, FREE, f.cacheable, f.mode, f.comp, None, &mut repr);
    Val::Map(repr.into())
}

fn generate_free_prim(f: FreeStaticPrimFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    if f.prim.is_extension {
        generate_extension(
            f.prim.id.clone(),
            false,
            FREE,
            f.cacheable,
            f.mode.clone(),
            &mut repr,
        );
    } else {
        repr.insert(symbol(ID), Val::Symbol(f.prim.id.clone()));
    }
    Val::Map(repr.into())
}

fn generate_free_comp(f: FreeStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    generate_composite(
        false,
        FREE,
        f.cacheable,
        f.mode.clone(),
        f.comp.clone(),
        None,
        &mut repr,
    );
    Val::Map(repr.into())
}

fn generate_const_prim(f: ConstStaticPrimFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    if f.prim.is_extension {
        generate_extension(
            f.prim.id.clone(),
            false,
            CONST,
            f.cacheable,
            f.mode.clone(),
            &mut repr,
        );
    } else {
        repr.insert(symbol(ID), Val::Symbol(f.prim.id.clone()));
    }
    Val::Map(repr.into())
}

fn generate_const_comp(f: ConstStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    generate_composite(
        false,
        CONST,
        f.cacheable,
        f.mode.clone(),
        f.comp.clone(),
        Some(f.ctx_name.clone()),
        &mut repr,
    );
    Val::Map(repr.into())
}

fn generate_mut_prim(f: MutStaticPrimFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    if f.prim.is_extension {
        generate_extension(
            f.prim.id.clone(),
            false,
            MUTABLE,
            f.cacheable,
            f.mode.clone(),
            &mut repr,
        );
    } else {
        repr.insert(symbol(ID), Val::Symbol(f.prim.id.clone()));
    }
    Val::Map(repr.into())
}

fn generate_mut_comp(f: MutStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    generate_composite(
        false,
        MUTABLE,
        f.cacheable,
        f.mode.clone(),
        f.comp.clone(),
        Some(f.ctx_name.clone()),
        &mut repr,
    );
    Val::Map(repr.into())
}

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
    composite: Composite,
    ctx_name: Option<Symbol>,
    repr: &mut Map<Val, Val>,
) {
    generate_func_common(cell, access, cacheable, mode, repr);
    if composite.body_mode != Mode::default() {
        let mode = Val::Func(FuncVal::Mode(ModeFunc::new(composite.body_mode).into()));
        repr.insert(symbol(BODY_MODE), mode);
    }
    if composite.body != Val::default() {
        repr.insert(symbol(BODY), composite.body);
    }
    if composite.prelude != Ctx::default() {
        repr.insert(symbol(PRELUDE), Val::Ctx(CtxVal::from(composite.prelude)));
    }
    if &*composite.input_name != DEFAULT_INPUT_NAME {
        repr.insert(symbol(INPUT_NAME), Val::Symbol(composite.input_name));
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
