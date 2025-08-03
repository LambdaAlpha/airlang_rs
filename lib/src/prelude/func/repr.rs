use log::error;

use crate::prelude::FuncMode;
use crate::prelude::mode::Mode;
use crate::prelude::mode::SymbolMode;
use crate::prelude::mode::TaskPrimMode;
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
const CALL_SETUP: &str = "call_setup";
const SOLVE_SETUP: &str = "solve_setup";
// todo rename
const CTX_ACCESS: &str = "context_access";
const CELL: &str = "cell";

const FREE: &str = "free";
const CONST: &str = "constant";
const MUTABLE: &str = "mutable";

pub(super) fn parse_mode() -> Mode {
    let mut map = Map::default();
    map.insert(symbol(CODE), FuncMode::prim_mode(SymbolMode::Ref, TaskPrimMode::Form));
    FuncMode::map_mode(map, FuncMode::default_mode())
}

// todo design defaults
pub(super) fn parse_func(input: Val) -> Option<FuncVal> {
    let Val::Map(mut map) = input else {
        error!("{input:?} should be a map");
        return None;
    };

    let id = parse_id(map_remove(&mut map, ID))?;
    // todo design
    let FuncCode { ctx_name, input_name, body } = parse_code(map_remove(&mut map, CODE))?;
    let ctx = parse_ctx(map_remove(&mut map, CTX))?;
    let setup =
        parse_task_setup(map_remove(&mut map, CALL_SETUP), map_remove(&mut map, SOLVE_SETUP))?;
    let ctx_access = map_remove(&mut map, CTX_ACCESS);
    let ctx_access = parse_ctx_access(&ctx_access)?;
    let cell = parse_cell(map_remove(&mut map, CELL))?;
    let free_comp = FreeComposite { body, input_name };
    let func = match ctx_access {
        FREE => {
            if cell {
                let func = FreeCellCompFunc { id, comp: free_comp, ctx, setup };
                FuncVal::FreeCellComp(FreeCellCompFuncVal::from(func))
            } else {
                let func = FreeStaticCompFunc { id, comp: free_comp, ctx, setup };
                FuncVal::FreeStaticComp(FreeStaticCompFuncVal::from(func))
            }
        }
        CONST => {
            let ctx_name = ctx_name?;
            let comp = DynComposite { free: free_comp, ctx_name };
            if cell {
                let func = ConstCellCompFunc { id, comp, ctx, setup };
                FuncVal::ConstCellComp(ConstCellCompFuncVal::from(func))
            } else {
                let func = ConstStaticCompFunc { id, comp, ctx, setup };
                FuncVal::ConstStaticComp(ConstStaticCompFuncVal::from(func))
            }
        }
        MUTABLE => {
            let ctx_name = ctx_name?;
            let comp = DynComposite { free: free_comp, ctx_name };
            if cell {
                let func = MutCellCompFunc { id, comp, ctx, setup };
                FuncVal::MutCellComp(MutCellCompFuncVal::from(func))
            } else {
                let func = MutStaticCompFunc { id, comp, ctx, setup };
                FuncVal::MutStaticComp(MutStaticCompFuncVal::from(func))
            }
        }
        s => {
            error!("ctx access {s} should be one of {FREE}, {CONST}, or {MUTABLE}");
            return None;
        }
    };
    Some(func)
}

fn parse_id(id: Val) -> Option<Symbol> {
    match id {
        Val::Unit(_) => Some(Symbol::default()),
        Val::Symbol(id) => Some(id),
        _ => None,
    }
}

struct FuncCode {
    ctx_name: Option<Symbol>,
    input_name: Symbol,
    body: Val,
}

fn parse_code(code: Val) -> Option<FuncCode> {
    let code = match code {
        Val::Unit(_) => FuncCode {
            ctx_name: Some(Symbol::default()),
            input_name: Symbol::default(),
            body: Val::default(),
        },
        Val::Pair(names_body) => {
            let names_body = Pair::from(names_body);
            match names_body.first {
                Val::Pair(ctx_input) => {
                    let Val::Symbol(ctx) = &ctx_input.first else {
                        error!("ctx {:?} should be a symbol", ctx_input.first);
                        return None;
                    };
                    let Val::Symbol(input) = &ctx_input.second else {
                        error!("input {:?} should be a symbol", ctx_input.second);
                        return None;
                    };
                    FuncCode {
                        ctx_name: Some(ctx.clone()),
                        input_name: input.clone(),
                        body: names_body.second,
                    }
                }
                Val::Symbol(input) => {
                    FuncCode { ctx_name: None, input_name: input, body: names_body.second }
                }
                v => {
                    error!("name {v:?} should be a symbol or a pair of symbol");
                    return None;
                }
            }
        }
        v => {
            error!("code {v:?} should be a pair or a unit");
            return None;
        }
    };
    Some(code)
}

fn parse_ctx(ctx: Val) -> Option<Ctx> {
    match ctx {
        Val::Ctx(ctx) => Some(Ctx::from(ctx)),
        Val::Unit(_) => Some(Ctx::default()),
        v => {
            error!("ctx {v:?} should be a ctx or a unit");
            None
        }
    }
}

fn parse_task_setup(call: Val, solve: Val) -> Option<Setup> {
    let call = parse_setup(call)?;
    let solve = parse_setup(solve)?;
    Some(Setup { call, solve })
}

fn parse_setup(setup: Val) -> Option<Option<FuncVal>> {
    match setup {
        Val::Unit(_) => Some(None),
        Val::Func(func) => Some(Some(func)),
        v => {
            error!("setup {v:?} should be a function or a unit");
            None
        }
    }
}

fn parse_ctx_access(access: &Val) -> Option<&str> {
    match &access {
        Val::Symbol(s) => Some(&**s),
        Val::Unit(_) => Some(MUTABLE),
        v => {
            error!("ctx access {v:?} should be a symbol or a unit");
            None
        }
    }
}

fn parse_cell(cell: Val) -> Option<bool> {
    match cell {
        Val::Unit(_) => Some(false),
        Val::Bit(b) => Some(*b),
        v => {
            error!("cell {v:?} should be a bit or a unit");
            None
        }
    }
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
    let comp = CompRepr { id: f.id, access: FREE, cell: true, setup: f.setup, ctx: f.ctx };
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
        id: f.id.clone(),
        access: FREE,
        cell: false,
        setup: f.setup.clone(),
        ctx: f.ctx.clone(),
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
    let comp = CompRepr { id: f.id, access: CONST, cell: true, setup: f.setup, ctx: f.ctx };
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
        id: f.id.clone(),
        access: CONST,
        cell: false,
        setup: f.setup.clone(),
        ctx: f.ctx.clone(),
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
    let comp = CompRepr { id: f.id, access: MUTABLE, cell: true, setup: f.setup, ctx: f.ctx };
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
        id: f.id.clone(),
        access: MUTABLE,
        cell: false,
        setup: f.setup.clone(),
        ctx: f.ctx.clone(),
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
    id: Symbol,
    access: &'static str,
    cell: bool,
    setup: Setup,
    ctx: Ctx,
}

fn generate_comp(repr: &mut Map<Val, Val>, comp: CompRepr) {
    if !comp.id.is_empty() {
        repr.insert(symbol(ID), Val::Symbol(comp.id));
    }
    if comp.cell {
        repr.insert(symbol(CELL), Val::Bit(Bit::true_()));
    }
    if comp.access != MUTABLE {
        repr.insert(symbol(CTX_ACCESS), symbol(comp.access));
    }
    let call_setup = generate_setup(comp.setup.call);
    if !call_setup.is_unit() {
        repr.insert(symbol(CALL_SETUP), call_setup);
    }
    let solve_setup = generate_setup(comp.setup.solve);
    if !solve_setup.is_unit() {
        repr.insert(symbol(SOLVE_SETUP), solve_setup);
    }
    if comp.ctx != Ctx::default() {
        repr.insert(symbol(CTX), Val::Ctx(CtxVal::from(comp.ctx)));
    }
}

pub(super) fn generate_setup(setup: Option<FuncVal>) -> Val {
    match setup {
        Some(func) => Val::Func(func),
        None => Val::default(),
    }
}

pub(super) fn generate_ctx_access(ctx_access: CtxAccess) -> &'static str {
    match ctx_access {
        CtxAccess::Free => FREE,
        CtxAccess::Const => CONST,
        CtxAccess::Mut => MUTABLE,
    }
}
