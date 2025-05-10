use crate::Bit;
use crate::CodeMode;
use crate::ConstStaticCompFunc;
use crate::ConstStaticCompFuncVal;
use crate::ConstStaticPrimFuncVal;
use crate::Ctx;
use crate::CtxVal;
use crate::FreeCellCompFunc;
use crate::FreeCellCompFuncVal;
use crate::FreeCellPrimFunc;
use crate::FreeCellPrimFuncVal;
use crate::FreeStaticCompFunc;
use crate::FreeStaticCompFuncVal;
use crate::FreeStaticPrimFuncVal;
use crate::FuncVal;
use crate::Map;
use crate::Mode;
use crate::ModeFunc;
use crate::MutStaticCompFunc;
use crate::MutStaticCompFuncVal;
use crate::MutStaticPrimFuncVal;
use crate::Pair;
use crate::Symbol;
use crate::SymbolMode;
use crate::Val;
use crate::func::FuncMode;
use crate::func::FuncTrait;
use crate::func::comp::Composite;
use crate::func::const_cell_comp::ConstCellCompFunc;
use crate::func::const_cell_prim::ConstCellPrimFunc;
use crate::func::mut_cell_comp::MutCellCompFunc;
use crate::func::mut_cell_prim::MutCellPrimFunc;
use crate::mode::repr::generate;
use crate::prelude::find_prelude;
use crate::utils::val::map_remove;
use crate::utils::val::symbol;
use crate::val::func::const_cell_comp::ConstCellCompFuncVal;
use crate::val::func::const_cell_prim::ConstCellPrimFuncVal;
use crate::val::func::mut_cell_comp::MutCellCompFuncVal;
use crate::val::func::mut_cell_prim::MutCellPrimFuncVal;

pub(crate) const CODE: &str = "code";
pub(crate) const CTX: &str = "context";
pub(crate) const ID: &str = "id";
pub(crate) const FORWARD_MODE: &str = "forward_mode";
pub(crate) const REVERSE_MODE: &str = "reverse_mode";
pub(crate) const CTX_ACCESS: &str = "context_access";
pub(crate) const CELL: &str = "cell";

pub(crate) const FREE: &str = "free";
pub(crate) const CONST: &str = "constant";
pub(crate) const MUTABLE: &str = "mutable";

pub(crate) fn parse_mode() -> Option<Mode> {
    let mut map = Map::default();
    map.insert(symbol(ID), FuncMode::uni_mode(CodeMode::Form, SymbolMode::Literal));
    map.insert(symbol(CODE), FuncMode::uni_mode(CodeMode::Form, SymbolMode::Ref));
    map.insert(symbol(CTX_ACCESS), FuncMode::symbol_mode(SymbolMode::Literal));
    FuncMode::map_mode(map, FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode())
}

pub(crate) fn parse_func(input: Val) -> Option<FuncVal> {
    let Val::Map(mut map) = input else {
        return None;
    };

    match map_remove(&mut map, ID) {
        Val::Symbol(id) => return find_prelude_func(id),
        Val::Unit(_) => {}
        _ => return None,
    }

    let (ctx_name, input_name, body) = match map_remove(&mut map, CODE) {
        Val::Unit(_) => (Some(Symbol::default()), Symbol::default(), Val::default()),
        Val::Pair(names_body) => {
            let names_body = Pair::from(names_body);
            match names_body.first {
                Val::Pair(ctx_input) => {
                    let Val::Symbol(ctx) = &ctx_input.first else {
                        return None;
                    };
                    let Val::Symbol(input) = &ctx_input.second else {
                        return None;
                    };
                    (Some(ctx.clone()), input.clone(), names_body.second)
                }
                Val::Symbol(input) => (None, input, names_body.second),
                _ => return None,
            }
        }
        _ => return None,
    };
    let ctx = match map_remove(&mut map, CTX) {
        Val::Ctx(ctx) => Ctx::from(ctx),
        Val::Unit(_) => Ctx::default(),
        _ => return None,
    };
    let forward = match map_remove(&mut map, FORWARD_MODE) {
        Val::Unit(_) => FuncMode::default_mode(),
        Val::Func(FuncVal::Mode(call_mode)) => call_mode.inner().clone(),
        _ => return None,
    };
    let reverse = match map_remove(&mut map, REVERSE_MODE) {
        Val::Unit(_) => FuncMode::default_mode(),
        Val::Func(FuncVal::Mode(call_mode)) => call_mode.inner().clone(),
        _ => return None,
    };
    let mode = FuncMode { forward, reverse };
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

fn find_prelude_func(id: Symbol) -> Option<FuncVal> {
    let Val::Func(func) = find_prelude(id)? else {
        return None;
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
    generate_primitive_prelude(f.prim.id)
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
    generate_primitive_prelude(f.prim.id.clone())
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
    generate_primitive_prelude(f.prim.id)
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
    generate_primitive_prelude(f.prim.id.clone())
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
    generate_primitive_prelude(f.prim.id)
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
    generate_primitive_prelude(f.prim.id.clone())
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

fn generate_call<F: FuncTrait>(repr: &mut Map<Val, Val>, func: &F) {
    let code = func.code();
    if !code.is_unit() {
        repr.insert(symbol(CODE), code);
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
    if common.mode.forward != FuncMode::default_mode() {
        let forward_mode = Val::Func(FuncVal::Mode(ModeFunc::new(common.mode.forward).into()));
        repr.insert(symbol(FORWARD_MODE), forward_mode);
    }
    if common.mode.reverse != FuncMode::default_mode() {
        let reverse_mode = Val::Func(FuncVal::Mode(ModeFunc::new(common.mode.reverse).into()));
        repr.insert(symbol(REVERSE_MODE), reverse_mode);
    }
}
