use log::error;

use crate::cfg::prelude::FuncMode;
use crate::cfg::prelude::mode::Mode;
use crate::cfg::prelude::mode::SymbolMode;
use crate::cfg::prelude::mode::TaskPrimMode;
use crate::cfg::prelude::utils::map_remove;
use crate::cfg::prelude::utils::symbol;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::CtxAccess;
use crate::semantics::func::ConstCompFunc;
use crate::semantics::func::DynComposite;
use crate::semantics::func::FreeCompFunc;
use crate::semantics::func::FreeComposite;
use crate::semantics::func::MutCompFunc;
use crate::semantics::func::Setup;
use crate::semantics::val::ConstCompFuncVal;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::CtxVal;
use crate::semantics::val::FreeCompFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutCompFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
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

const FREE: &str = "free";
const CONST: &str = "constant";
const MUTABLE: &str = "mutable";

pub(in crate::cfg) fn parse_mode() -> Mode {
    let mut map = Map::default();
    map.insert(symbol(CODE), FuncMode::prim_mode(SymbolMode::Ref, TaskPrimMode::Form));
    FuncMode::map_mode(map, FuncMode::default_mode())
}

// todo design defaults
pub(in crate::cfg) fn parse_func(input: Val) -> Option<FuncVal> {
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
    let free_comp = FreeComposite { body, input_name };
    let func = match ctx_access {
        FREE => {
            let func = FreeCompFunc { id, comp: free_comp, ctx, setup };
            FuncVal::FreeComp(FreeCompFuncVal::from(func))
        }
        CONST => {
            let ctx_name = ctx_name?;
            let comp = DynComposite { free: free_comp, ctx_name };
            let func = ConstCompFunc { id, comp, ctx, setup };
            FuncVal::ConstComp(ConstCompFuncVal::from(func))
        }
        MUTABLE => {
            let ctx_name = ctx_name?;
            let comp = DynComposite { free: free_comp, ctx_name };
            let func = MutCompFunc { id, comp, ctx, setup };
            FuncVal::MutComp(MutCompFuncVal::from(func))
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

pub(in crate::cfg) fn generate_func(f: FuncVal) -> Val {
    match f {
        FuncVal::FreePrim(f) => generate_free_prim(f),
        FuncVal::FreeComp(f) => generate_free_comp(f),
        FuncVal::ConstPrim(f) => generate_const_prim(f),
        FuncVal::ConstComp(f) => generate_const_comp(f),
        FuncVal::MutPrim(f) => generate_mut_prim(f),
        FuncVal::MutComp(f) => generate_mut_comp(f),
    }
}

fn generate_free_prim(f: FreePrimFuncVal) -> Val {
    generate_prim(f.id.clone())
}

fn generate_free_comp(f: FreeCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(CODE), free_code(&f.comp));
    let comp =
        CompRepr { id: f.id.clone(), access: FREE, setup: f.setup.clone(), ctx: f.ctx.clone() };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_const_prim(f: ConstPrimFuncVal) -> Val {
    generate_prim(f.id.clone())
}

fn generate_const_comp(f: ConstCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(CODE), dyn_code(&f.comp));
    let comp =
        CompRepr { id: f.id.clone(), access: CONST, setup: f.setup.clone(), ctx: f.ctx.clone() };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_mut_prim(f: MutPrimFuncVal) -> Val {
    generate_prim(f.id.clone())
}

fn generate_mut_comp(f: MutCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(CODE), dyn_code(&f.comp));
    let comp =
        CompRepr { id: f.id.clone(), access: MUTABLE, setup: f.setup.clone(), ctx: f.ctx.clone() };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_prim(id: Symbol) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(symbol(ID), Val::Symbol(id));
    Val::Map(repr.into())
}

pub(in crate::cfg) fn generate_code(func: &FuncVal) -> Val {
    match func {
        FuncVal::FreePrim(_) => Val::default(),
        FuncVal::FreeComp(f) => free_code(&f.comp),
        FuncVal::ConstPrim(_) => Val::default(),
        FuncVal::ConstComp(f) => dyn_code(&f.comp),
        FuncVal::MutPrim(_) => Val::default(),
        FuncVal::MutComp(f) => dyn_code(&f.comp),
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
    setup: Setup,
    ctx: Ctx,
}

fn generate_comp(repr: &mut Map<Val, Val>, comp: CompRepr) {
    if !comp.id.is_empty() {
        repr.insert(symbol(ID), Val::Symbol(comp.id));
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

pub(in crate::cfg) fn generate_setup(setup: Option<FuncVal>) -> Val {
    match setup {
        Some(func) => Val::Func(func),
        None => Val::default(),
    }
}

pub(in crate::cfg) fn generate_ctx_access(ctx_access: CtxAccess) -> &'static str {
    match ctx_access {
        CtxAccess::Free => FREE,
        CtxAccess::Const => CONST,
        CtxAccess::Mut => MUTABLE,
    }
}
