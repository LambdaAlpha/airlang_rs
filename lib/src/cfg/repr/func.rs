use std::rc::Rc;

use crate::bug;
use crate::cfg::lib::func::NEW;
use crate::cfg::utils::map_remove;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::CompCtx;
use crate::semantics::func::CompFn;
use crate::semantics::func::CompFunc;
use crate::semantics::func::PrimCtx;
use crate::semantics::val::CompFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::PrimFuncVal;
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
const CTX_FREE: &str = "context_free";
const CTX_CONST: &str = "context_constant";

// todo design defaults
pub(in crate::cfg) fn parse_func(cfg: &mut Cfg, input: Val) -> Option<FuncVal> {
    let Val::Map(mut map) = input else {
        bug!(cfg, "{NEW}: expected input to be a map, but got {input}");
        return None;
    };
    let raw_input = parse_bit(RAW_INPUT, cfg, map_remove(&mut map, RAW_INPUT))?;
    // todo design
    let CompCode { ctx_name, input_name, body } = parse_code(cfg, map_remove(&mut map, CODE))?;
    let prelude = map_remove(&mut map, PRELUDE);
    let ctx_free = parse_bit(CTX_FREE, cfg, map_remove(&mut map, CTX_FREE))?;
    let ctx = if ctx_free {
        CompCtx::Free
    } else {
        let Some(name) = ctx_name else {
            bug!(cfg, "{NEW}: mutable context function need a context name");
            return None;
        };
        let const_ = parse_bit(CTX_CONST, cfg, map_remove(&mut map, CTX_CONST))?;
        CompCtx::Default { name, const_ }
    };
    let comp = CompFn { prelude, body, input_name, ctx };
    let func = CompFunc { raw_input, fn_: comp };
    let func = FuncVal::Comp(CompFuncVal::from(func));
    Some(func)
}

fn parse_bit(tag: &str, cfg: &mut Cfg, val: Val) -> Option<bool> {
    match val {
        Val::Unit(_) => Some(false),
        Val::Bit(bit) => Some(bit.into()),
        v => {
            bug!(cfg, "{NEW}: expected {tag} to be a unit or a bit, but got {v}");
            None
        },
    }
}

pub(in crate::cfg) fn generate_func(f: FuncVal) -> Val {
    match f {
        FuncVal::Prim(f) => generate_prim(f),
        FuncVal::Comp(f) => generate_comp(f),
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
                },
                Val::Key(input) => {
                    CompCode { ctx_name: None, input_name: input, body: names_body.right }
                },
                v => {
                    bug!(cfg, "{NEW}: expected names to be a key or a pair of key, but got {v}");
                    return None;
                },
            }
        },
        v => {
            bug!(cfg, "{NEW}: expected {CODE} to be a pair or a unit, but got {v}");
            return None;
        },
    };
    Some(code)
}

pub(in crate::cfg) fn generate_code(func: &FuncVal) -> Val {
    match func {
        FuncVal::Prim(f) => prim_code(&f.fn_),
        FuncVal::Comp(f) => comp_code(&f.fn_),
    }
}

fn prim_code<T: ?Sized>(fn_: &Rc<T>) -> Val {
    let ptr = Rc::as_ptr(fn_).addr();
    let int = Int::from(ptr);
    Val::Int(int.into())
}

fn comp_code(comp: &CompFn) -> Val {
    let input = Val::Key(comp.input_name.clone());
    let names = if let CompCtx::Default { name, .. } = &comp.ctx {
        let ctx = Val::Key(name.clone());
        Val::Pair(Pair::new(ctx, input).into())
    } else {
        input
    };
    Val::Pair(Pair::new(names, comp.body.clone()).into())
}

fn generate_prim(f: PrimFuncVal) -> Val {
    prim(PrimRepr {
        common: CommonRepr { ctx: f.ctx, raw_input: f.raw_input, code: prim_code(&f.fn_) },
    })
}

fn generate_comp(f: CompFuncVal) -> Val {
    comp(CompRepr {
        common: CommonRepr {
            ctx: f.fn_.ctx.to_prim_ctx(),
            raw_input: f.raw_input,
            code: comp_code(&f.fn_),
        },
        prelude: f.fn_.prelude.clone(),
    })
}

struct CommonRepr {
    ctx: PrimCtx,
    raw_input: bool,
    code: Val,
}

fn generate_common(repr: &mut Map<Key, Val>, common: CommonRepr) {
    repr.insert(Key::from_str_unchecked(RAW_INPUT), Val::Bit(Bit::from(common.raw_input)));
    repr.insert(Key::from_str_unchecked(CODE), common.code);
    match common.ctx {
        PrimCtx::Free => {
            repr.insert(Key::from_str_unchecked(CTX_FREE), Val::Bit(Bit::true_()));
        },
        PrimCtx::Const_ => {
            repr.insert(Key::from_str_unchecked(CTX_FREE), Val::Bit(Bit::false_()));
            repr.insert(Key::from_str_unchecked(CTX_CONST), Val::Bit(Bit::true_()));
        },
        PrimCtx::Mut => {
            repr.insert(Key::from_str_unchecked(CTX_FREE), Val::Bit(Bit::false_()));
            repr.insert(Key::from_str_unchecked(CTX_CONST), Val::Bit(Bit::false_()));
        },
    }
}

struct PrimRepr {
    common: CommonRepr,
}

fn prim(prim: PrimRepr) -> Val {
    let mut repr = Map::<Key, Val>::default();
    generate_common(&mut repr, prim.common);
    Val::Map(repr.into())
}

struct CompRepr {
    common: CommonRepr,
    prelude: Val,
}

fn comp(comp: CompRepr) -> Val {
    let mut repr = Map::<Key, Val>::default();
    generate_common(&mut repr, comp.common);
    repr.insert(Key::from_str_unchecked(PRELUDE), comp.prelude);
    Val::Map(repr.into())
}
