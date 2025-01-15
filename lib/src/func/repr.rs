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
    Symbol,
    Val,
    func::{
        FuncMode,
        comp::Composite,
        const_cell_comp::ConstCellCompFunc,
        const_cell_prim::ConstCellPrimFunc,
        mut_cell_comp::MutCellCompFunc,
        mut_cell_prim::MutCellPrimFunc,
    },
    mode::{
        repr::generate,
        symbol::PrefixMode,
    },
    prelude::{
        form_mode,
        map_mode,
        symbol_literal_mode,
    },
    utils::val::{
        map_remove,
        symbol,
    },
    val::func::{
        const_cell_comp::ConstCellCompFuncVal,
        const_cell_prim::ConstCellPrimFuncVal,
        mut_cell_comp::MutCellCompFuncVal,
        mut_cell_prim::MutCellPrimFuncVal,
    },
};

pub(crate) const BODY_MODE: &str = "body_mode";
pub(crate) const BODY: &str = "body";
pub(crate) const CTX: &str = "context";
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
    map.insert(symbol(BODY), form_mode(PrefixMode::Ref));
    map.insert(symbol(CTX), Mode::default());
    map.insert(symbol(INPUT_NAME), symbol_literal_mode());
    map.insert(symbol(CTX_NAME), symbol_literal_mode());
    map.insert(symbol(CTX_ACCESS), symbol_literal_mode());
    map.insert(symbol(CALL_MODE), Mode::default());
    map.insert(symbol(ABSTRACT_MODE), Mode::default());
    map.insert(symbol(ASK_MODE), Mode::default());
    map.insert(symbol(CACHEABLE), Mode::default());
    map.insert(symbol(CELL), Mode::default());
    map_mode(map, Mode::default(), Mode::default())
}

pub(crate) fn generate_mode() -> Mode {
    let mut map = Map::default();
    map.insert(symbol(BODY_MODE), Mode::default());
    map.insert(symbol(BODY), form_mode(PrefixMode::Ref));
    map.insert(symbol(CTX), Mode::default());
    map.insert(symbol(INPUT_NAME), symbol_literal_mode());
    map.insert(symbol(CTX_NAME), symbol_literal_mode());
    map.insert(symbol(CTX_ACCESS), symbol_literal_mode());
    map.insert(symbol(CALL_MODE), Mode::default());
    map.insert(symbol(ABSTRACT_MODE), Mode::default());
    map.insert(symbol(ASK_MODE), Mode::default());
    map.insert(symbol(CACHEABLE), Mode::default());
    map.insert(symbol(ID), Mode::default());
    map.insert(symbol(IS_EXTENSION), Mode::default());
    map.insert(symbol(CELL), Mode::default());
    map_mode(map, Mode::default(), Mode::default())
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
    let ctx = match map_remove(&mut map, CTX) {
        Val::Ctx(ctx) => Ctx::from(ctx),
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
        ctx,
        input_name,
    };
    let func = match ctx_access {
        FREE => {
            if cell {
                let func = FreeCellCompFunc::new(comp, mode, cacheable);
                FuncVal::FreeCellComp(FreeCellCompFuncVal::from(func))
            } else {
                let func = FreeStaticCompFunc::new(comp, mode, cacheable);
                FuncVal::FreeStaticComp(FreeStaticCompFuncVal::from(func))
            }
        }
        CONST => {
            if cell {
                let func = ConstCellCompFunc::new(comp, ctx_name, mode, cacheable);
                FuncVal::ConstCellComp(ConstCellCompFuncVal::from(func))
            } else {
                let func = ConstStaticCompFunc::new(comp, ctx_name, mode, cacheable);
                FuncVal::ConstStaticComp(ConstStaticCompFuncVal::from(func))
            }
        }
        MUTABLE => {
            if cell {
                let func = MutCellCompFunc::new(comp, ctx_name, mode, cacheable);
                FuncVal::MutCellComp(MutCellCompFuncVal::from(func))
            } else {
                let func = MutStaticCompFunc::new(comp, ctx_name, mode, cacheable);
                FuncVal::MutStaticComp(MutStaticCompFuncVal::from(func))
            }
        }
        _ => return None,
    };
    Some(func)
}

pub(crate) fn generate_func(f: FuncVal) -> Val {
    match f {
        FuncVal::Mode(f) => generate(f.self_mode()),
        FuncVal::FreeCellPrim(f) => generate_free_cell_prim(f),
        FuncVal::FreeCellComp(f) => generate_free_cell_comp(f),
        FuncVal::FreeStaticPrim(f) => generate_free_static_prim(f),
        FuncVal::FreeStaticComp(f) => generate_free_static_comp(f),
        FuncVal::ConstCellPrim(f) => generate_const_cell_prim(f),
        FuncVal::ConstCellComp(f) => generate_const_cell_comp(f),
        FuncVal::ConstStaticPrim(f) => generate_const_static_prim(f),
        FuncVal::ConstStaticComp(f) => generate_const_static_comp(f),
        FuncVal::MutCellPrim(f) => generate_mut_cell_prim(f),
        FuncVal::MutCellComp(f) => generate_mut_cell_comp(f),
        FuncVal::MutStaticPrim(f) => generate_mut_static_prim(f),
        FuncVal::MutStaticComp(f) => generate_mut_static_comp(f),
    }
}

fn generate_free_cell_prim(f: FreeCellPrimFuncVal) -> Val {
    let f = FreeCellPrimFunc::from(f);
    if f.prim.is_extension {
        let common = FuncCommon {
            access: FREE,
            cell: true,
            mode: f.mode,
            cacheable: f.cacheable,
        };
        generate_primitive_extension(f.prim.id, common)
    } else {
        generate_primitive_prelude(f.prim.id)
    }
}

fn generate_free_cell_comp(f: FreeCellCompFuncVal) -> Val {
    let f = FreeCellCompFunc::from(f);
    let common = FuncCommon {
        access: FREE,
        cell: true,
        mode: f.mode,
        cacheable: f.cacheable,
    };
    let mut repr = Map::<Val, Val>::default();
    generate_func_common(&mut repr, common);
    generate_composite(&mut repr, f.comp);
    Val::Map(repr.into())
}

fn generate_free_static_prim(f: FreeStaticPrimFuncVal) -> Val {
    if f.prim.is_extension {
        let common = FuncCommon {
            access: FREE,
            cell: false,
            mode: f.mode.clone(),
            cacheable: f.cacheable,
        };
        generate_primitive_extension(f.prim.id.clone(), common)
    } else {
        generate_primitive_prelude(f.prim.id.clone())
    }
}

fn generate_free_static_comp(f: FreeStaticCompFuncVal) -> Val {
    let common = FuncCommon {
        access: FREE,
        cell: false,
        mode: f.mode.clone(),
        cacheable: f.cacheable,
    };
    let mut repr = Map::<Val, Val>::default();
    generate_func_common(&mut repr, common);
    generate_composite(&mut repr, f.comp.clone());
    Val::Map(repr.into())
}

fn generate_const_cell_prim(f: ConstCellPrimFuncVal) -> Val {
    let f = ConstCellPrimFunc::from(f);
    if f.prim.is_extension {
        let common = FuncCommon {
            access: CONST,
            cell: true,
            mode: f.mode,
            cacheable: f.cacheable,
        };
        generate_primitive_extension(f.prim.id, common)
    } else {
        generate_primitive_prelude(f.prim.id)
    }
}

fn generate_const_cell_comp(f: ConstCellCompFuncVal) -> Val {
    let f = ConstCellCompFunc::from(f);
    let common = FuncCommon {
        access: CONST,
        cell: true,
        mode: f.mode,
        cacheable: f.cacheable,
    };
    let mut repr = Map::<Val, Val>::default();
    generate_func_common(&mut repr, common);
    generate_composite(&mut repr, f.comp);
    generate_ctx_name(&mut repr, f.ctx_name);
    Val::Map(repr.into())
}

fn generate_const_static_prim(f: ConstStaticPrimFuncVal) -> Val {
    if f.prim.is_extension {
        let common = FuncCommon {
            access: CONST,
            cell: false,
            mode: f.mode.clone(),
            cacheable: f.cacheable,
        };
        generate_primitive_extension(f.prim.id.clone(), common)
    } else {
        generate_primitive_prelude(f.prim.id.clone())
    }
}

fn generate_const_static_comp(f: ConstStaticCompFuncVal) -> Val {
    let common = FuncCommon {
        access: CONST,
        cell: false,
        mode: f.mode.clone(),
        cacheable: f.cacheable,
    };
    let mut repr = Map::<Val, Val>::default();
    generate_func_common(&mut repr, common);
    generate_composite(&mut repr, f.comp.clone());
    generate_ctx_name(&mut repr, f.ctx_name.clone());
    Val::Map(repr.into())
}

fn generate_mut_cell_prim(f: MutCellPrimFuncVal) -> Val {
    let f = MutCellPrimFunc::from(f);
    if f.prim.is_extension {
        let common = FuncCommon {
            access: MUTABLE,
            cell: true,
            mode: f.mode,
            cacheable: f.cacheable,
        };
        generate_primitive_extension(f.prim.id, common)
    } else {
        generate_primitive_prelude(f.prim.id)
    }
}

fn generate_mut_cell_comp(f: MutCellCompFuncVal) -> Val {
    let f = MutCellCompFunc::from(f);
    let common = FuncCommon {
        access: MUTABLE,
        cell: true,
        mode: f.mode,
        cacheable: f.cacheable,
    };
    let mut repr = Map::<Val, Val>::default();
    generate_func_common(&mut repr, common);
    generate_composite(&mut repr, f.comp);
    generate_ctx_name(&mut repr, f.ctx_name);
    Val::Map(repr.into())
}

fn generate_mut_static_prim(f: MutStaticPrimFuncVal) -> Val {
    if f.prim.is_extension {
        let common = FuncCommon {
            access: MUTABLE,
            cell: false,
            mode: f.mode.clone(),
            cacheable: f.cacheable,
        };
        generate_primitive_extension(f.prim.id.clone(), common)
    } else {
        generate_primitive_prelude(f.prim.id.clone())
    }
}

fn generate_mut_static_comp(f: MutStaticCompFuncVal) -> Val {
    let common = FuncCommon {
        access: MUTABLE,
        cell: false,
        mode: f.mode.clone(),
        cacheable: f.cacheable,
    };
    let mut repr = Map::<Val, Val>::default();
    generate_func_common(&mut repr, common);
    generate_composite(&mut repr, f.comp.clone());
    generate_ctx_name(&mut repr, f.ctx_name.clone());
    Val::Map(repr.into())
}

fn generate_primitive_prelude(id: Symbol) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(ID), Val::Symbol(id));
    Val::Map(repr.into())
}

fn generate_primitive_extension(id: Symbol, common: FuncCommon) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(ID), Val::Symbol(id));
    repr.insert(symbol(IS_EXTENSION), Val::Bit(Bit::true1()));
    generate_func_common(&mut repr, common);
    Val::Map(repr.into())
}

fn generate_composite(repr: &mut Map<Val, Val>, composite: Composite) {
    if composite.body_mode != Mode::default() {
        let mode = Val::Func(FuncVal::Mode(ModeFunc::new(composite.body_mode).into()));
        repr.insert(symbol(BODY_MODE), mode);
    }
    if composite.body != Val::default() {
        repr.insert(symbol(BODY), composite.body);
    }
    if composite.ctx != Ctx::default() {
        repr.insert(symbol(CTX), Val::Ctx(CtxVal::from(composite.ctx)));
    }
    if &*composite.input_name != DEFAULT_INPUT_NAME {
        repr.insert(symbol(INPUT_NAME), Val::Symbol(composite.input_name));
    }
}

struct FuncCommon {
    access: &'static str,
    cell: bool,
    mode: FuncMode,
    cacheable: bool,
}

fn generate_func_common(repr: &mut Map<Val, Val>, common: FuncCommon) {
    if common.cell {
        repr.insert(symbol(CELL), Val::Bit(Bit::true1()));
    }
    if common.access != MUTABLE {
        repr.insert(symbol(CTX_ACCESS), symbol(common.access));
    }
    if common.cacheable {
        repr.insert(symbol(CACHEABLE), Val::Bit(Bit::new(true)));
    }
    if common.mode.call != Mode::default() {
        let call_mode = Val::Func(FuncVal::Mode(ModeFunc::new(common.mode.call).into()));
        repr.insert(symbol(CALL_MODE), call_mode);
    }
    if common.mode.abstract1 != Mode::default() {
        let abstract_mode = Val::Func(FuncVal::Mode(ModeFunc::new(common.mode.abstract1).into()));
        repr.insert(symbol(ABSTRACT_MODE), abstract_mode);
    }
    if common.mode.ask != Mode::default() {
        let ask_mode = Val::Func(FuncVal::Mode(ModeFunc::new(common.mode.ask).into()));
        repr.insert(symbol(ASK_MODE), ask_mode);
    }
}

fn generate_ctx_name(repr: &mut Map<Val, Val>, ctx_name: Symbol) {
    if &*ctx_name != DEFAULT_CTX_NAME {
        repr.insert(symbol(CTX_NAME), Val::Symbol(ctx_name));
    }
}
