use const_format::concatcp;
use log::error;

use crate::cfg::utils::key;
use crate::cfg::utils::map_remove;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::ctx::CtxAccess;
use crate::semantics::func::ConstCompFunc;
use crate::semantics::func::DynComposite;
use crate::semantics::func::FreeCompFunc;
use crate::semantics::func::FreeComposite;
use crate::semantics::func::MutCompFunc;
use crate::semantics::memo::Memo;
use crate::semantics::val::ConstCompFuncVal;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreeCompFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MemoVal;
use crate::semantics::val::MutCompFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::Pair;

// todo rename
const CODE: &str = "code";
const MEMO: &str = "memory";
const ID: &str = "id";
const RAW_INPUT: &str = "raw_input";
// todo rename
const CTX_ACCESS: &str = "context_access";

const FREE: &str = concatcp!(PREFIX_ID, "free");
const CONST: &str = concatcp!(PREFIX_ID, "constant");
const MUTABLE: &str = concatcp!(PREFIX_ID, "mutable");

// todo design defaults
pub(in crate::cfg) fn parse_func(input: Val) -> Option<FuncVal> {
    let Val::Map(mut map) = input else {
        error!("{input:?} should be a map");
        return None;
    };

    let id = parse_id(map_remove(&mut map, ID))?;
    let raw_input = parse_raw_input(map_remove(&mut map, RAW_INPUT))?;
    // todo design
    let FuncCode { ctx_name, input_name, body } = parse_code(map_remove(&mut map, CODE))?;
    let memo = parse_memo(map_remove(&mut map, MEMO))?;
    let ctx_access = map_remove(&mut map, CTX_ACCESS);
    let ctx_access = parse_ctx_access(&ctx_access)?;
    let free_comp = FreeComposite { body, input_name };
    let func = match ctx_access {
        FREE => {
            let func = FreeCompFunc { id, raw_input, comp: free_comp, memo };
            FuncVal::FreeComp(FreeCompFuncVal::from(func))
        }
        CONST => {
            let ctx_name = ctx_name?;
            let comp = DynComposite { free: free_comp, ctx_name };
            let func = ConstCompFunc { id, raw_input, comp, memo };
            FuncVal::ConstComp(ConstCompFuncVal::from(func))
        }
        MUTABLE => {
            let ctx_name = ctx_name?;
            let comp = DynComposite { free: free_comp, ctx_name };
            let func = MutCompFunc { id, raw_input, comp, memo };
            FuncVal::MutComp(MutCompFuncVal::from(func))
        }
        s => {
            error!("ctx access {s} should be one of {FREE}, {CONST}, or {MUTABLE}");
            return None;
        }
    };
    Some(func)
}

fn parse_id(id: Val) -> Option<Key> {
    match id {
        Val::Unit(_) => Some(Key::default()),
        Val::Key(id) => Some(id),
        _ => None,
    }
}

fn parse_raw_input(id: Val) -> Option<bool> {
    match id {
        Val::Unit(_) => Some(false),
        Val::Bit(bit) => Some(bit.into()),
        _ => None,
    }
}

struct FuncCode {
    ctx_name: Option<Key>,
    input_name: Key,
    body: Val,
}

fn parse_code(code: Val) -> Option<FuncCode> {
    let code = match code {
        Val::Unit(_) => FuncCode {
            ctx_name: Some(Key::default()),
            input_name: Key::default(),
            body: Val::default(),
        },
        Val::Pair(names_body) => {
            let names_body = Pair::from(names_body);
            match names_body.first {
                Val::Pair(ctx_input) => {
                    let Val::Key(ctx) = &ctx_input.first else {
                        error!("ctx {:?} should be a key", ctx_input.first);
                        return None;
                    };
                    let Val::Key(input) = &ctx_input.second else {
                        error!("input {:?} should be a key", ctx_input.second);
                        return None;
                    };
                    FuncCode {
                        ctx_name: Some(ctx.clone()),
                        input_name: input.clone(),
                        body: names_body.second,
                    }
                }
                Val::Key(input) => {
                    FuncCode { ctx_name: None, input_name: input, body: names_body.second }
                }
                v => {
                    error!("name {v:?} should be a key or a pair of key");
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

fn parse_memo(memo: Val) -> Option<Memo> {
    match memo {
        Val::Memo(memo) => Some(Memo::from(memo)),
        Val::Unit(_) => Some(Memo::default()),
        v => {
            error!("memo {v:?} should be a memo or a unit");
            None
        }
    }
}

fn parse_ctx_access(access: &Val) -> Option<&str> {
    match &access {
        Val::Key(s) => Some(&**s),
        Val::Unit(_) => Some(MUTABLE),
        v => {
            error!("ctx access {v:?} should be a key or a unit");
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
    repr.insert(key(CODE), free_code(&f.comp));
    let comp = CompRepr { id: f.id.clone(), access: FREE, memo: f.memo.clone() };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_const_prim(f: ConstPrimFuncVal) -> Val {
    generate_prim(f.id.clone())
}

fn generate_const_comp(f: ConstCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(key(CODE), dyn_code(&f.comp));
    let comp = CompRepr { id: f.id.clone(), access: CONST, memo: f.memo.clone() };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_mut_prim(f: MutPrimFuncVal) -> Val {
    generate_prim(f.id.clone())
}

fn generate_mut_comp(f: MutCompFuncVal) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(key(CODE), dyn_code(&f.comp));
    let comp = CompRepr { id: f.id.clone(), access: MUTABLE, memo: f.memo.clone() };
    generate_comp(&mut repr, comp);
    Val::Map(repr.into())
}

fn generate_prim(id: Key) -> Val {
    let mut repr = Map::<Val, Val>::default();
    repr.insert(key(ID), Val::Key(id));
    Val::Map(repr.into())
}

pub(in crate::cfg) fn generate_code(func: &FuncVal) -> Option<Val> {
    match func {
        FuncVal::FreePrim(_) => None,
        FuncVal::FreeComp(f) => Some(free_code(&f.comp)),
        FuncVal::ConstPrim(_) => None,
        FuncVal::ConstComp(f) => Some(dyn_code(&f.comp)),
        FuncVal::MutPrim(_) => None,
        FuncVal::MutComp(f) => Some(dyn_code(&f.comp)),
    }
}

fn free_code(comp: &FreeComposite) -> Val {
    let input = Val::Key(comp.input_name.clone());
    let output = comp.body.clone();
    Val::Pair(Pair::new(input, output).into())
}

fn dyn_code(comp: &DynComposite) -> Val {
    let ctx = Val::Key(comp.ctx_name.clone());
    let input = Val::Key(comp.free.input_name.clone());
    let names = Val::Pair(Pair::new(ctx, input).into());
    Val::Pair(Pair::new(names, comp.free.body.clone()).into())
}

struct CompRepr {
    id: Key,
    access: &'static str,
    memo: Memo,
}

fn generate_comp(repr: &mut Map<Val, Val>, comp: CompRepr) {
    if !comp.id.is_empty() {
        repr.insert(key(ID), Val::Key(comp.id));
    }
    if comp.access != MUTABLE {
        repr.insert(key(CTX_ACCESS), key(comp.access));
    }
    if comp.memo != Memo::default() {
        repr.insert(key(MEMO), Val::Memo(MemoVal::from(comp.memo)));
    }
}

pub(in crate::cfg) fn generate_ctx_access(ctx_access: CtxAccess) -> &'static str {
    match ctx_access {
        CtxAccess::Free => FREE,
        CtxAccess::Const => CONST,
        CtxAccess::Mut => MUTABLE,
    }
}
