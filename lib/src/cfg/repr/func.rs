use std::rc::Rc;

use const_format::concatcp;

use crate::bug;
use crate::cfg::lib::func::NEW;
use crate::cfg::utils::key;
use crate::cfg::utils::map_remove;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::ctx::CtxAccess;
use crate::semantics::func::ConstCompFunc;
use crate::semantics::func::DynComposite;
use crate::semantics::func::FreeCompFunc;
use crate::semantics::func::FreeComposite;
use crate::semantics::func::MutCompFunc;
use crate::semantics::val::ConstCompFuncVal;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreeCompFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutCompFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::Pair;

// todo rename
const CODE: &str = "code";
const PRELUDE: &str = "prelude";
const RAW_INPUT: &str = "raw_input";
// todo rename
const CTX_ACCESS: &str = "context_access";

const FREE: &str = concatcp!(PREFIX_ID, "free");
const CONST: &str = concatcp!(PREFIX_ID, "constant");
const MUTABLE: &str = concatcp!(PREFIX_ID, "mutable");

// todo design defaults
pub(in crate::cfg) fn parse_func(cfg: &mut Cfg, input: Val) -> Option<FuncVal> {
    let Val::Map(mut map) = input else {
        bug!(cfg, "{NEW}: expected input to be a map, but got {input}");
        return None;
    };
    let raw_input = parse_raw_input(cfg, map_remove(&mut map, RAW_INPUT))?;
    // todo design
    let CompCode { ctx_name, input_name, body } = parse_code(cfg, map_remove(&mut map, CODE))?;
    let prelude = map_remove(&mut map, PRELUDE);
    let ctx_access = map_remove(&mut map, CTX_ACCESS);
    let ctx_access = parse_ctx_access(cfg, &ctx_access)?;
    let func = match ctx_access {
        CtxAccess::Free => {
            let comp = FreeComposite { prelude, body, input_name };
            let func = FreeCompFunc { raw_input, comp };
            FuncVal::FreeComp(FreeCompFuncVal::from(func))
        }
        CtxAccess::Const => {
            let Some(ctx_name) = ctx_name else {
                bug!(cfg, "{NEW}: constant context function need a context name");
                return None;
            };
            let comp = DynComposite { prelude, body, input_name, ctx_name };
            let func = ConstCompFunc { raw_input, comp };
            FuncVal::ConstComp(ConstCompFuncVal::from(func))
        }
        CtxAccess::Mut => {
            let Some(ctx_name) = ctx_name else {
                bug!(cfg, "{NEW}: mutable context function need a context name");
                return None;
            };
            let comp = DynComposite { prelude, body, input_name, ctx_name };
            let func = MutCompFunc { raw_input, comp };
            FuncVal::MutComp(MutCompFuncVal::from(func))
        }
    };
    Some(func)
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

fn parse_raw_input(cfg: &mut Cfg, val: Val) -> Option<bool> {
    match val {
        Val::Unit(_) => Some(false),
        Val::Bit(bit) => Some(bit.into()),
        v => {
            bug!(cfg, "{NEW}: expected {RAW_INPUT} to be a unit or a bit, but got {v}");
            None
        }
    }
}

struct CompCode {
    ctx_name: Option<Key>,
    input_name: Key,
    body: Val,
}

fn parse_code(cfg: &mut Cfg, code: Val) -> Option<CompCode> {
    let code = match code {
        Val::Unit(_) => CompCode {
            ctx_name: Some(Key::default()),
            input_name: Key::default(),
            body: Val::default(),
        },
        Val::Pair(names_body) => {
            let names_body = Pair::from(names_body);
            match names_body.left {
                Val::Pair(ctx_input) => {
                    let Val::Key(ctx) = &ctx_input.left else {
                        bug!(
                            cfg,
                            "{NEW}: expected context name to be a key, but got {}",
                            ctx_input.left
                        );
                        return None;
                    };
                    let Val::Key(input) = &ctx_input.right else {
                        bug!(
                            cfg,
                            "{NEW}: expected input name to be a key, but got {}",
                            ctx_input.right
                        );
                        return None;
                    };
                    CompCode {
                        ctx_name: Some(ctx.clone()),
                        input_name: input.clone(),
                        body: names_body.right,
                    }
                }
                Val::Key(input) => {
                    CompCode { ctx_name: None, input_name: input, body: names_body.right }
                }
                v => {
                    bug!(cfg, "{NEW}: expected names to be a key or a pair of key, but got {v}");
                    return None;
                }
            }
        }
        v => {
            bug!(cfg, "{NEW}: expected {CODE} to be a pair or a unit, but got {v}");
            return None;
        }
    };
    Some(code)
}

pub(in crate::cfg) fn generate_code(func: &FuncVal) -> Val {
    match func {
        FuncVal::FreePrim(f) => prim_code(&f.fn_),
        FuncVal::FreeComp(f) => free_code(&f.comp),
        FuncVal::ConstPrim(f) => prim_code(&f.fn_),
        FuncVal::ConstComp(f) => dyn_code(&f.comp),
        FuncVal::MutPrim(f) => prim_code(&f.fn_),
        FuncVal::MutComp(f) => dyn_code(&f.comp),
    }
}

fn prim_code<T: ?Sized>(fn_: &Rc<T>) -> Val {
    let ptr = Rc::as_ptr(fn_).addr();
    let int = Int::from(ptr);
    Val::Int(int.into())
}

fn free_code(comp: &FreeComposite) -> Val {
    let input = Val::Key(comp.input_name.clone());
    let output = comp.body.clone();
    Val::Pair(Pair::new(input, output).into())
}

fn dyn_code(comp: &DynComposite) -> Val {
    let ctx = Val::Key(comp.ctx_name.clone());
    let input = Val::Key(comp.input_name.clone());
    let names = Val::Pair(Pair::new(ctx, input).into());
    Val::Pair(Pair::new(names, comp.body.clone()).into())
}

fn parse_ctx_access(cfg: &mut Cfg, access: &Val) -> Option<CtxAccess> {
    match &access {
        Val::Key(s) => match &**s {
            FREE => Some(CtxAccess::Free),
            CONST => Some(CtxAccess::Const),
            MUTABLE => Some(CtxAccess::Mut),
            s => {
                bug!(
                    cfg,
                    "{NEW}: expected {CTX_ACCESS} to be one of {FREE}, {CONST}, or {MUTABLE}, but got {s}"
                );
                None
            }
        },
        Val::Unit(_) => Some(CtxAccess::Mut),
        v => {
            bug!(cfg, "{NEW}: expected {CTX_ACCESS} to be a key or a unit, but got {v}");
            None
        }
    }
}

pub(in crate::cfg) fn generate_ctx_access(ctx_access: CtxAccess) -> &'static str {
    match ctx_access {
        CtxAccess::Free => FREE,
        CtxAccess::Const => CONST,
        CtxAccess::Mut => MUTABLE,
    }
}

fn generate_free_prim(f: FreePrimFuncVal) -> Val {
    generate_prim(PrimRepr {
        common: CommonRepr { access: FREE, raw_input: f.raw_input, code: prim_code(&f.fn_) },
    })
}

fn generate_free_comp(f: FreeCompFuncVal) -> Val {
    generate_comp(CompRepr {
        common: CommonRepr { access: FREE, raw_input: f.raw_input, code: free_code(&f.comp) },
        prelude: f.comp.prelude.clone(),
    })
}

fn generate_const_prim(f: ConstPrimFuncVal) -> Val {
    generate_prim(PrimRepr {
        common: CommonRepr { access: CONST, raw_input: f.raw_input, code: prim_code(&f.fn_) },
    })
}

fn generate_const_comp(f: ConstCompFuncVal) -> Val {
    generate_comp(CompRepr {
        common: CommonRepr { access: CONST, raw_input: f.raw_input, code: dyn_code(&f.comp) },
        prelude: f.comp.prelude.clone(),
    })
}

fn generate_mut_prim(f: MutPrimFuncVal) -> Val {
    generate_prim(PrimRepr {
        common: CommonRepr { access: MUTABLE, raw_input: f.raw_input, code: prim_code(&f.fn_) },
    })
}

fn generate_mut_comp(f: MutCompFuncVal) -> Val {
    generate_comp(CompRepr {
        common: CommonRepr { access: MUTABLE, raw_input: f.raw_input, code: dyn_code(&f.comp) },
        prelude: f.comp.prelude.clone(),
    })
}

struct CommonRepr {
    access: &'static str,
    raw_input: bool,
    code: Val,
}

fn generate_common(repr: &mut Map<Key, Val>, common: CommonRepr) {
    repr.insert(Key::from_str_unchecked(CTX_ACCESS), key(common.access));
    repr.insert(Key::from_str_unchecked(RAW_INPUT), Val::Bit(Bit::from(common.raw_input)));
    repr.insert(Key::from_str_unchecked(CODE), common.code);
}

struct PrimRepr {
    common: CommonRepr,
}

fn generate_prim(prim: PrimRepr) -> Val {
    let mut repr = Map::<Key, Val>::default();
    generate_common(&mut repr, prim.common);
    Val::Map(repr.into())
}

struct CompRepr {
    common: CommonRepr,
    prelude: Val,
}

fn generate_comp(comp: CompRepr) -> Val {
    let mut repr = Map::<Key, Val>::default();
    generate_common(&mut repr, comp.common);
    repr.insert(Key::from_str_unchecked(PRELUDE), comp.prelude);
    Val::Map(repr.into())
}
