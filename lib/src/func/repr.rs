use crate::{
    Bit,
    Change,
    ChangeMode,
    CodeMode,
    CompMode,
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
    Pair,
    PairMode,
    Symbol,
    SymbolMode,
    Val,
    func::{
        FuncMode,
        FuncTrait,
        comp::Composite,
        const_cell_comp::ConstCellCompFunc,
        const_cell_prim::ConstCellPrimFunc,
        mut_cell_comp::MutCellCompFunc,
        mut_cell_prim::MutCellPrimFunc,
    },
    mode::repr::generate,
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

pub(crate) const CALL: &str = "call";
pub(crate) const CTX: &str = "context";
pub(crate) const ID: &str = "id";
pub(crate) const IS_EXTENSION: &str = "is_extension";
pub(crate) const CALL_MODE: &str = "call_mode";
pub(crate) const OPTIMIZE_MODE: &str = "optimize_mode";
pub(crate) const SOLVE_MODE: &str = "solve_mode";
pub(crate) const CTX_ACCESS: &str = "context_access";
pub(crate) const CELL: &str = "cell";

pub(crate) const FREE: &str = "free";
pub(crate) const CONST: &str = "constant";
pub(crate) const MUTABLE: &str = "mutable";

pub(crate) fn parse_mode() -> Option<Mode> {
    let mut map = Map::default();
    map.insert(symbol(CALL), func_call_mode());
    map.insert(symbol(CTX_ACCESS), FuncMode::symbol_mode(SymbolMode::Literal));
    FuncMode::map_mode(map, FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode())
}

pub(crate) fn generate_mode() -> Option<Mode> {
    let mut map = Map::default();
    map.insert(symbol(CALL), func_call_mode());
    map.insert(symbol(CTX_ACCESS), FuncMode::symbol_mode(SymbolMode::Literal));
    FuncMode::map_mode(map, FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode())
}

fn func_call_mode() -> Option<Mode> {
    let symbol_literal = FuncMode::symbol_mode(SymbolMode::Literal);
    let form_ref = FuncMode::uni_mode(CodeMode::Form, SymbolMode::Ref);
    Some(Mode::Comp(Box::new(CompMode {
        pair: Some(PairMode {
            pair: Pair::new(
                symbol_literal.clone(),
                FuncMode::change_mode(symbol_literal.clone(), form_ref.clone()),
            ),
        }),
        change: Some(ChangeMode { change: Change::new(symbol_literal, form_ref) }),
        ..CompMode::from(FuncMode::default_uni_mode())
    })))
}

pub(crate) fn parse_func(input: Val) -> Option<FuncVal> {
    let Val::Map(mut map) = input else {
        return None;
    };
    let (ctx_name, input_name, body) = match map_remove(&mut map, CALL) {
        Val::Unit(_) => (Some(Symbol::default()), Symbol::default(), Val::default()),
        Val::Pair(ctx_change) => {
            let ctx_change = Pair::from(ctx_change);
            let Val::Symbol(ctx) = ctx_change.first else {
                return None;
            };
            let Val::Change(change) = ctx_change.second else {
                return None;
            };
            let change = Change::from(change);
            let Val::Symbol(input) = change.from else {
                return None;
            };
            (Some(ctx), input, change.to)
        }
        Val::Change(change) => {
            let change = Change::from(change);
            let Val::Symbol(input) = change.from else {
                return None;
            };
            (None, input, change.to)
        }
        _ => return None,
    };
    let ctx = match map_remove(&mut map, CTX) {
        Val::Ctx(ctx) => Ctx::from(ctx),
        Val::Unit(_) => Ctx::default(),
        _ => return None,
    };
    let call = match map_remove(&mut map, CALL_MODE) {
        Val::Unit(_) => FuncMode::default_mode(),
        Val::Func(FuncVal::Mode(call_mode)) => call_mode.inner().clone(),
        _ => return None,
    };
    let optimize = match map_remove(&mut map, OPTIMIZE_MODE) {
        Val::Unit(_) => FuncMode::default_mode(),
        Val::Func(FuncVal::Mode(optimize_mode)) => optimize_mode.inner().clone(),
        _ => return None,
    };
    let solve = match map_remove(&mut map, SOLVE_MODE) {
        Val::Unit(_) => FuncMode::default_mode(),
        Val::Func(FuncVal::Mode(solve_mode)) => solve_mode.inner().clone(),
        _ => return None,
    };
    let mode = FuncMode { call, optimize, solve };
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
    let comp = Composite { body, ctx, input_name };
    let func = match ctx_access {
        FREE => {
            if cell {
                let func = FreeCellCompFunc::new(comp, mode);
                FuncVal::FreeCellComp(FreeCellCompFuncVal::from(func))
            } else {
                let func = FreeStaticCompFunc::new(comp, mode);
                FuncVal::FreeStaticComp(FreeStaticCompFuncVal::from(func))
            }
        }
        CONST => {
            let ctx_name = ctx_name?;
            if cell {
                let func = ConstCellCompFunc::new(comp, ctx_name, mode);
                FuncVal::ConstCellComp(ConstCellCompFuncVal::from(func))
            } else {
                let func = ConstStaticCompFunc::new(comp, ctx_name, mode);
                FuncVal::ConstStaticComp(ConstStaticCompFuncVal::from(func))
            }
        }
        MUTABLE => {
            let ctx_name = ctx_name?;
            if cell {
                let func = MutCellCompFunc::new(comp, ctx_name, mode);
                FuncVal::MutCellComp(MutCellCompFuncVal::from(func))
            } else {
                let func = MutStaticCompFunc::new(comp, ctx_name, mode);
                FuncVal::MutStaticComp(MutStaticCompFuncVal::from(func))
            }
        }
        _ => return None,
    };
    Some(func)
}

pub(crate) fn generate_func(f: FuncVal) -> Val {
    match f {
        FuncVal::Mode(f) => generate(f.inner()),
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
        let common = FuncCommon { access: FREE, cell: true, mode: f.mode };
        generate_primitive_extension(f.prim.id, common)
    } else {
        generate_primitive_prelude(f.prim.id)
    }
}

fn generate_free_cell_comp(f: FreeCellCompFuncVal) -> Val {
    let f = FreeCellCompFunc::from(f);
    let mut repr = Map::<Val, Val>::default();
    generate_call(&mut repr, &f);
    let common = FuncCommon { access: FREE, cell: true, mode: f.mode };
    generate_func_common(&mut repr, common);
    generate_ctx(&mut repr, f.comp.ctx);
    Val::Map(repr.into())
}

fn generate_free_static_prim(f: FreeStaticPrimFuncVal) -> Val {
    if f.prim.is_extension {
        let common = FuncCommon { access: FREE, cell: false, mode: f.mode.clone() };
        generate_primitive_extension(f.prim.id.clone(), common)
    } else {
        generate_primitive_prelude(f.prim.id.clone())
    }
}

fn generate_free_static_comp(f: FreeStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    generate_call(&mut repr, &f);
    let common = FuncCommon { access: FREE, cell: false, mode: f.mode.clone() };
    generate_func_common(&mut repr, common);
    generate_ctx(&mut repr, f.comp.ctx.clone());
    Val::Map(repr.into())
}

fn generate_const_cell_prim(f: ConstCellPrimFuncVal) -> Val {
    let f = ConstCellPrimFunc::from(f);
    if f.prim.is_extension {
        let common = FuncCommon { access: CONST, cell: true, mode: f.mode };
        generate_primitive_extension(f.prim.id, common)
    } else {
        generate_primitive_prelude(f.prim.id)
    }
}

fn generate_const_cell_comp(f: ConstCellCompFuncVal) -> Val {
    let f = ConstCellCompFunc::from(f);
    let mut repr = Map::<Val, Val>::default();
    generate_call(&mut repr, &f);
    let common = FuncCommon { access: CONST, cell: true, mode: f.mode };
    generate_func_common(&mut repr, common);
    generate_ctx(&mut repr, f.comp.ctx);
    Val::Map(repr.into())
}

fn generate_const_static_prim(f: ConstStaticPrimFuncVal) -> Val {
    if f.prim.is_extension {
        let common = FuncCommon { access: CONST, cell: false, mode: f.mode.clone() };
        generate_primitive_extension(f.prim.id.clone(), common)
    } else {
        generate_primitive_prelude(f.prim.id.clone())
    }
}

fn generate_const_static_comp(f: ConstStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    generate_call(&mut repr, &f);
    let common = FuncCommon { access: CONST, cell: false, mode: f.mode.clone() };
    generate_func_common(&mut repr, common);
    generate_ctx(&mut repr, f.comp.ctx.clone());
    Val::Map(repr.into())
}

fn generate_mut_cell_prim(f: MutCellPrimFuncVal) -> Val {
    let f = MutCellPrimFunc::from(f);
    if f.prim.is_extension {
        let common = FuncCommon { access: MUTABLE, cell: true, mode: f.mode };
        generate_primitive_extension(f.prim.id, common)
    } else {
        generate_primitive_prelude(f.prim.id)
    }
}

fn generate_mut_cell_comp(f: MutCellCompFuncVal) -> Val {
    let f = MutCellCompFunc::from(f);
    let mut repr = Map::<Val, Val>::default();
    generate_call(&mut repr, &f);
    let common = FuncCommon { access: MUTABLE, cell: true, mode: f.mode };
    generate_func_common(&mut repr, common);
    generate_ctx(&mut repr, f.comp.ctx);
    Val::Map(repr.into())
}

fn generate_mut_static_prim(f: MutStaticPrimFuncVal) -> Val {
    if f.prim.is_extension {
        let common = FuncCommon { access: MUTABLE, cell: false, mode: f.mode.clone() };
        generate_primitive_extension(f.prim.id.clone(), common)
    } else {
        generate_primitive_prelude(f.prim.id.clone())
    }
}

fn generate_mut_static_comp(f: MutStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    generate_call(&mut repr, &f);
    let common = FuncCommon { access: MUTABLE, cell: false, mode: f.mode.clone() };
    generate_func_common(&mut repr, common);
    generate_ctx(&mut repr, f.comp.ctx.clone());
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

fn generate_call<F: FuncTrait>(repr: &mut Map<Val, Val>, func: &F) {
    let call = func.call();
    if !call.is_unit() {
        repr.insert(symbol(CALL), call);
    }
}

fn generate_ctx(repr: &mut Map<Val, Val>, ctx: Ctx) {
    if ctx != Ctx::default() {
        repr.insert(symbol(CTX), Val::Ctx(CtxVal::from(ctx)));
    }
}

struct FuncCommon {
    access: &'static str,
    cell: bool,
    mode: FuncMode,
}

fn generate_func_common(repr: &mut Map<Val, Val>, common: FuncCommon) {
    if common.cell {
        repr.insert(symbol(CELL), Val::Bit(Bit::true1()));
    }
    if common.access != MUTABLE {
        repr.insert(symbol(CTX_ACCESS), symbol(common.access));
    }
    if common.mode.call != FuncMode::default_mode() {
        let call_mode = Val::Func(FuncVal::Mode(ModeFunc::new(common.mode.call).into()));
        repr.insert(symbol(CALL_MODE), call_mode);
    }
    if common.mode.optimize != FuncMode::default_mode() {
        let optimize_mode = Val::Func(FuncVal::Mode(ModeFunc::new(common.mode.optimize).into()));
        repr.insert(symbol(OPTIMIZE_MODE), optimize_mode);
    }
    if common.mode.solve != FuncMode::default_mode() {
        let solve_mode = Val::Func(FuncVal::Mode(ModeFunc::new(common.mode.solve).into()));
        repr.insert(symbol(SOLVE_MODE), solve_mode);
    }
}
