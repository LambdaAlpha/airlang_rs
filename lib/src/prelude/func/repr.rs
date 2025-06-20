use crate::prelude::FuncMode;
use crate::prelude::mode::CodeMode;
use crate::prelude::mode::Mode;
use crate::prelude::mode::SymbolMode;
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
use crate::semantics::func::MutCellCompFunc;
use crate::semantics::func::MutCellPrimFunc;
use crate::semantics::func::MutStaticCompFunc;
use crate::semantics::func::Setup;
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
const SETUP: &str = "setup";
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
    let setup = match map_remove(&mut map, SETUP) {
        Val::Unit(_) => None,
        Val::Pair(pair) => {
            let pair = Pair::from(pair);
            let forward = match pair.first {
                Val::Unit(_) => {
                    FuncVal::MutStaticPrim(FuncMode::mode_into_mut_func(FuncMode::default_mode()))
                }
                Val::Func(func) => func,
                _ => return None,
            };
            let reverse = match pair.second {
                Val::Unit(_) => {
                    FuncVal::MutStaticPrim(FuncMode::mode_into_mut_func(FuncMode::default_mode()))
                }
                Val::Func(func) => func,
                _ => return None,
            };
            Some(Setup { forward, reverse })
        }
        _ => return None,
    };
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
                let func = FreeCellCompFunc { comp: free_comp, ctx, setup };
                FuncVal::FreeCellComp(FreeCellCompFuncVal::from(func))
            } else {
                let func = FreeStaticCompFunc { comp: free_comp, ctx, setup };
                FuncVal::FreeStaticComp(FreeStaticCompFuncVal::from(func))
            }
        }
        CONST => {
            let ctx_name = ctx_name?;
            let comp = DynComposite { free: free_comp, ctx_name };
            if cell {
                let func = ConstCellCompFunc { comp, ctx, setup, ctx_explicit };
                FuncVal::ConstCellComp(ConstCellCompFuncVal::from(func))
            } else {
                let func = ConstStaticCompFunc { comp, ctx, setup, ctx_explicit };
                FuncVal::ConstStaticComp(ConstStaticCompFuncVal::from(func))
            }
        }
        MUTABLE => {
            let ctx_name = ctx_name?;
            let comp = DynComposite { free: free_comp, ctx_name };
            if cell {
                let func = MutCellCompFunc { comp, ctx, setup, ctx_explicit };
                FuncVal::MutCellComp(MutCellCompFuncVal::from(func))
            } else {
                let func = MutStaticCompFunc { comp, ctx, setup, ctx_explicit };
                FuncVal::MutStaticComp(MutStaticCompFuncVal::from(func))
            }
        }
        _ => return None,
    };
    Some(func)
}

pub(super) fn generate_func(f: FuncVal) -> Val {
    match f {
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
    repr.insert(symbol(CODE), free_code(&f.comp));
    let comp =
        CompRepr { access: FREE, cell: true, setup: f.setup, ctx: f.ctx, ctx_explicit: false };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_free_static_prim(f: FreeStaticPrimFuncVal) -> Val {
    generate_prim(f.id.clone())
}

fn generate_free_static_comp(f: FreeStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(CODE), free_code(&f.comp));
    let comp = CompRepr {
        access: FREE,
        cell: false,
        setup: f.setup.clone(),
        ctx: f.ctx.clone(),
        ctx_explicit: false,
    };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_const_cell_prim(f: ConstCellPrimFuncVal) -> Val {
    let f = ConstCellPrimFunc::from(f);
    generate_prim(f.id)
}

fn generate_const_cell_comp(f: ConstCellCompFuncVal) -> Val {
    let f = ConstCellCompFunc::from(f);
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(CODE), dyn_code(&f.comp));
    let comp = CompRepr {
        access: CONST,
        cell: true,
        setup: f.setup,
        ctx: f.ctx,
        ctx_explicit: f.ctx_explicit,
    };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_const_static_prim(f: ConstStaticPrimFuncVal) -> Val {
    generate_prim(f.id.clone())
}

fn generate_const_static_comp(f: ConstStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(CODE), dyn_code(&f.comp));
    let comp = CompRepr {
        access: CONST,
        cell: false,
        setup: f.setup.clone(),
        ctx: f.ctx.clone(),
        ctx_explicit: f.ctx_explicit,
    };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_mut_cell_prim(f: MutCellPrimFuncVal) -> Val {
    let f = MutCellPrimFunc::from(f);
    generate_prim(f.id)
}

fn generate_mut_cell_comp(f: MutCellCompFuncVal) -> Val {
    let f = MutCellCompFunc::from(f);
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(CODE), dyn_code(&f.comp));
    let comp = CompRepr {
        access: MUTABLE,
        cell: true,
        setup: f.setup,
        ctx: f.ctx,
        ctx_explicit: f.ctx_explicit,
    };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_mut_static_prim(f: MutStaticPrimFuncVal) -> Val {
    generate_prim(f.id.clone())
}

fn generate_mut_static_comp(f: MutStaticCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(CODE), dyn_code(&f.comp));
    let comp = CompRepr {
        access: MUTABLE,
        cell: false,
        setup: f.setup.clone(),
        ctx: f.ctx.clone(),
        ctx_explicit: f.ctx_explicit,
    };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_prim(id: Symbol) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(ID), Val::Symbol(id));
    Val::Map(repr.into())
}

pub(in crate::prelude) fn generate_code(func: &FuncVal) -> Val {
    match func {
        FuncVal::FreeCellPrim(_) => Val::default(),
        FuncVal::FreeCellComp(f) => free_code(&f.comp),
        FuncVal::FreeStaticPrim(_) => Val::default(),
        FuncVal::FreeStaticComp(f) => free_code(&f.comp),
        FuncVal::ConstCellPrim(_) => Val::default(),
        FuncVal::ConstCellComp(f) => dyn_code(&f.comp),
        FuncVal::ConstStaticPrim(_) => Val::default(),
        FuncVal::ConstStaticComp(f) => dyn_code(&f.comp),
        FuncVal::MutCellPrim(_) => Val::default(),
        FuncVal::MutCellComp(f) => dyn_code(&f.comp),
        FuncVal::MutStaticPrim(_) => Val::default(),
        FuncVal::MutStaticComp(f) => dyn_code(&f.comp),
    }
}

fn free_code(comp: &FreeComposite) -> Val {
    let input = Val::Symbol(comp.input_name.clone());
    let output = comp.body.clone();
    Val::Pair(Pair::new(input, output).into())
}

fn dyn_code(comp: &DynComposite) -> Val {
    let ctx = Val::Symbol(comp.ctx_name.clone());
    let input = Val::Symbol(comp.free.input_name.clone());
    let names = Val::Pair(Pair::new(ctx, input).into());
    Val::Pair(Pair::new(names, comp.free.body.clone()).into())
}

struct CompRepr {
    ctx_explicit: bool,
    access: &'static str,
    cell: bool,
    setup: Option<Setup>,
    ctx: Ctx,
}

fn generate_comp(repr: &mut Map<Val, Val>, comp: CompRepr) {
    if comp.ctx_explicit {
        repr.insert(symbol(CTX_EXPLICIT), Val::Bit(Bit::true_()));
    }
    if comp.cell {
        repr.insert(symbol(CELL), Val::Bit(Bit::true_()));
    }
    if comp.access != MUTABLE {
        repr.insert(symbol(CTX_ACCESS), symbol(comp.access));
    }
    if let Some(setup) = comp.setup {
        let forward = Val::Func(setup.forward);
        let reverse = Val::Func(setup.reverse);
        let pair = Val::Pair(Pair::new(forward, reverse).into());
        repr.insert(symbol(SETUP), pair);
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
