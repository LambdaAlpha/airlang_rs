use self::mode::generate;
use crate::prelude::find_prelude;
use crate::prelude::utils::map_remove;
use crate::prelude::utils::symbol;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::CtxAccess;
use crate::semantics::func::ConstCellCompFunc;
use crate::semantics::func::ConstCellPrimFunc;
use crate::semantics::func::ConstStaticCompFunc;
use crate::semantics::func::DynComposite;
use crate::semantics::func::FreeCellCompFunc;
use crate::semantics::func::FreeCellPrimFunc;
use crate::semantics::func::FreeComposite;
use crate::semantics::func::FreeStaticCompFunc;
use crate::semantics::func::FuncMode;
use crate::semantics::func::FuncTrait;
use crate::semantics::func::ModeFunc;
use crate::semantics::func::MutCellCompFunc;
use crate::semantics::func::MutCellPrimFunc;
use crate::semantics::func::MutStaticCompFunc;
use crate::semantics::mode::CodeMode;
use crate::semantics::mode::Mode;
use crate::semantics::mode::SymbolMode;
use crate::semantics::val::ConstCellCompFuncVal;
use crate::semantics::val::ConstCellPrimFuncVal;
use crate::semantics::val::ConstStaticCompFuncVal;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::CtxVal;
use crate::semantics::val::FreeCellCompFuncVal;
use crate::semantics::val::FreeCellPrimFuncVal;
use crate::semantics::val::FreeStaticCompFuncVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutCellCompFuncVal;
use crate::semantics::val::MutCellPrimFuncVal;
use crate::semantics::val::MutStaticCompFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;

// todo rename
const CODE: &str = "code";
const CTX: &str = "context";
const ID: &str = "id";
// todo rename
const FORWARD_MODE: &str = "forward_mode";
// todo rename
const REVERSE_MODE: &str = "reverse_mode";
// todo rename
const CTX_ACCESS: &str = "context_access";
const CELL: &str = "cell";
// todo rename
const CTX_EXPLICIT: &str = "context_explicit";

const FREE: &str = "free";
const CONST: &str = "constant";
const MUTABLE: &str = "mutable";

pub(super) fn parse_mode() -> Option<Mode> {
    let mut map = Map::default();
    map.insert(symbol(ID), FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form));
    map.insert(symbol(CODE), FuncMode::prim_mode(SymbolMode::Ref, CodeMode::Form));
    map.insert(symbol(CTX_ACCESS), FuncMode::symbol_mode(SymbolMode::Literal));
    FuncMode::map_mode(map, FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode())
}

// todo design defaults
pub(super) fn parse_func(input: Val) -> Option<FuncVal> {
    let Val::Map(mut map) = input else {
        return None;
    };

    match map_remove(&mut map, ID) {
        Val::Symbol(id) => return find_prelude_func(id),
        Val::Unit(_) => {}
        _ => return None,
    }

    // todo design
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
    let ctx_explicit = match map_remove(&mut map, CTX_EXPLICIT) {
        Val::Unit(_) => false,
        Val::Bit(b) => b.bool(),
        _ => return None,
    };
    let cell = match map_remove(&mut map, CELL) {
        Val::Unit(_) => false,
        Val::Bit(b) => b.bool(),
        _ => return None,
    };
    let free_comp = FreeComposite { body, input_name };
    let func = match ctx_access {
        FREE => {
            if cell {
                let func = FreeCellCompFunc { comp: free_comp, ctx, mode };
                FuncVal::FreeCellComp(FreeCellCompFuncVal::from(func))
            } else {
                let func = FreeStaticCompFunc { comp: free_comp, ctx, mode };
                FuncVal::FreeStaticComp(FreeStaticCompFuncVal::from(func))
            }
        }
        CONST => {
            let ctx_name = ctx_name?;
            let comp = DynComposite { free: free_comp, ctx_name };
            if cell {
                let func = ConstCellCompFunc { comp, ctx, mode, ctx_explicit };
                FuncVal::ConstCellComp(ConstCellCompFuncVal::from(func))
            } else {
                let func = ConstStaticCompFunc { comp, ctx, mode, ctx_explicit };
                FuncVal::ConstStaticComp(ConstStaticCompFuncVal::from(func))
            }
        }
        MUTABLE => {
            let ctx_name = ctx_name?;
            let comp = DynComposite { free: free_comp, ctx_name };
            if cell {
                let func = MutCellCompFunc { comp, ctx, mode, ctx_explicit };
                FuncVal::MutCellComp(MutCellCompFuncVal::from(func))
            } else {
                let func = MutStaticCompFunc { comp, ctx, mode, ctx_explicit };
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

pub(super) fn generate_func(f: FuncVal) -> Val {
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
    generate_prim(f.id)
}

fn generate_free_cell_comp(f: FreeCellCompFuncVal) -> Val {
    let f = FreeCellCompFunc::from(f);
    let mut repr = Map::<Val, Val>::default();
    generate_by_ref(&mut repr, &f);
    let comp = CompRepr { access: FREE, cell: true, mode: f.mode, ctx: f.ctx };
    generate_by_own(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_free_static_prim(f: FreeStaticPrimFuncVal) -> Val {
    generate_prim(f.id.clone())
}

fn generate_free_static_comp(f: FreeStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    generate_by_ref(&mut repr, &f);
    let comp = CompRepr { access: FREE, cell: false, mode: f.mode.clone(), ctx: f.ctx.clone() };
    generate_by_own(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_const_cell_prim(f: ConstCellPrimFuncVal) -> Val {
    let f = ConstCellPrimFunc::from(f);
    generate_prim(f.id)
}

fn generate_const_cell_comp(f: ConstCellCompFuncVal) -> Val {
    let f = ConstCellCompFunc::from(f);
    let mut repr = Map::<Val, Val>::default();
    generate_by_ref(&mut repr, &f);
    let comp = CompRepr { access: CONST, cell: true, mode: f.mode, ctx: f.ctx };
    generate_by_own(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_const_static_prim(f: ConstStaticPrimFuncVal) -> Val {
    generate_prim(f.id.clone())
}

fn generate_const_static_comp(f: ConstStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    generate_by_ref(&mut repr, &f);
    let comp = CompRepr { access: CONST, cell: false, mode: f.mode.clone(), ctx: f.ctx.clone() };
    generate_by_own(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_mut_cell_prim(f: MutCellPrimFuncVal) -> Val {
    let f = MutCellPrimFunc::from(f);
    generate_prim(f.id)
}

fn generate_mut_cell_comp(f: MutCellCompFuncVal) -> Val {
    let f = MutCellCompFunc::from(f);
    let mut repr = Map::<Val, Val>::default();
    generate_by_ref(&mut repr, &f);
    let comp = CompRepr { access: MUTABLE, cell: true, mode: f.mode, ctx: f.ctx };
    generate_by_own(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_mut_static_prim(f: MutStaticPrimFuncVal) -> Val {
    generate_prim(f.id.clone())
}

fn generate_mut_static_comp(f: MutStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    generate_by_ref(&mut repr, &f);
    let comp = CompRepr { access: MUTABLE, cell: false, mode: f.mode.clone(), ctx: f.ctx.clone() };
    generate_by_own(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_prim(id: Symbol) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(ID), Val::Symbol(id));
    Val::Map(repr.into())
}

fn generate_by_ref<F: FuncTrait>(repr: &mut Map<Val, Val>, func: &F) {
    let code = func.code();
    if !code.is_unit() {
        repr.insert(symbol(CODE), code);
    }
    if func.ctx_explicit() {
        repr.insert(symbol(CTX_EXPLICIT), Val::Bit(Bit::true_()));
    }
}

struct CompRepr {
    access: &'static str,
    cell: bool,
    mode: FuncMode,
    ctx: Ctx,
}

fn generate_by_own(repr: &mut Map<Val, Val>, comp: CompRepr) {
    if comp.cell {
        repr.insert(symbol(CELL), Val::Bit(Bit::true_()));
    }
    if comp.access != MUTABLE {
        repr.insert(symbol(CTX_ACCESS), symbol(comp.access));
    }
    if comp.mode.forward != FuncMode::default_mode() {
        let forward_mode = Val::Func(FuncVal::Mode(ModeFunc::new(comp.mode.forward).into()));
        repr.insert(symbol(FORWARD_MODE), forward_mode);
    }
    if comp.mode.reverse != FuncMode::default_mode() {
        let reverse_mode = Val::Func(FuncVal::Mode(ModeFunc::new(comp.mode.reverse).into()));
        repr.insert(symbol(REVERSE_MODE), reverse_mode);
    }
    if comp.ctx != Ctx::default() {
        repr.insert(symbol(CTX), Val::Ctx(CtxVal::from(comp.ctx)));
    }
}

pub(super) fn generate_ctx_access(ctx_access: CtxAccess) -> &'static str {
    match ctx_access {
        CtxAccess::Free => FREE,
        CtxAccess::Const => CONST,
        CtxAccess::Mut => MUTABLE,
    }
}

pub(super) mod mode;
