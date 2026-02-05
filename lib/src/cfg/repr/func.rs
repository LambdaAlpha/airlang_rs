use std::rc::Rc;

use crate::bug;
use crate::cfg::lib::func::NEW;
use crate::cfg::utils::map_remove;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::CtxCompFunc;
use crate::semantics::func::DynComposite;
use crate::semantics::func::FreeCompFunc;
use crate::semantics::func::FreeComposite;
use crate::semantics::val::CtxCompFuncVal;
use crate::semantics::val::CtxPrimFuncVal;
use crate::semantics::val::FreeCompFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
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
    let func = if ctx_free {
        let comp = FreeComposite { prelude, body, input_name };
        let func = FreeCompFunc { raw_input, comp };
        FuncVal::FreeComp(FreeCompFuncVal::from(func))
    } else {
        let Some(ctx_name) = ctx_name else {
            bug!(cfg, "{NEW}: mutable context function need a context name");
            return None;
        };
        let const_ = parse_bit(CTX_CONST, cfg, map_remove(&mut map, CTX_CONST))?;
        let comp = DynComposite { prelude, body, input_name, ctx_name, const_ };
        let func = CtxCompFunc { raw_input, comp };
        FuncVal::CtxComp(CtxCompFuncVal::from(func))
    };
    Some(func)
}

pub(in crate::cfg) fn generate_func(f: FuncVal) -> Val {
    match f {
        FuncVal::FreePrim(f) => generate_free_prim(f),
        FuncVal::FreeComp(f) => generate_free_comp(f),
        FuncVal::CtxPrim(f) => generate_ctx_prim(f),
        FuncVal::CtxComp(f) => generate_ctx_comp(f),
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
        FuncVal::FreePrim(f) => prim_code(&f.fn_),
        FuncVal::FreeComp(f) => free_code(&f.comp),
        FuncVal::CtxPrim(f) => prim_code(&f.fn_),
        FuncVal::CtxComp(f) => dyn_code(&f.comp),
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

fn generate_free_prim(f: FreePrimFuncVal) -> Val {
    generate_prim(PrimRepr {
        common: CommonRepr {
            free: true,
            const_: true,
            raw_input: f.raw_input,
            code: prim_code(&f.fn_),
        },
    })
}

fn generate_free_comp(f: FreeCompFuncVal) -> Val {
    generate_comp(CompRepr {
        common: CommonRepr {
            free: true,
            const_: true,
            raw_input: f.raw_input,
            code: free_code(&f.comp),
        },
        prelude: f.comp.prelude.clone(),
    })
}

fn generate_ctx_prim(f: CtxPrimFuncVal) -> Val {
    generate_prim(PrimRepr {
        common: CommonRepr {
            free: false,
            const_: f.const_,
            raw_input: f.raw_input,
            code: prim_code(&f.fn_),
        },
    })
}

fn generate_ctx_comp(f: CtxCompFuncVal) -> Val {
    generate_comp(CompRepr {
        common: CommonRepr {
            free: false,
            const_: f.comp.const_,
            raw_input: f.raw_input,
            code: dyn_code(&f.comp),
        },
        prelude: f.comp.prelude.clone(),
    })
}

struct CommonRepr {
    free: bool,
    const_: bool,
    raw_input: bool,
    code: Val,
}

fn generate_common(repr: &mut Map<Key, Val>, common: CommonRepr) {
    repr.insert(Key::from_str_unchecked(CTX_FREE), Val::Bit(Bit::from(common.free)));
    repr.insert(Key::from_str_unchecked(RAW_INPUT), Val::Bit(Bit::from(common.raw_input)));
    repr.insert(Key::from_str_unchecked(CODE), common.code);
    if !common.free {
        repr.insert(Key::from_str_unchecked(CTX_CONST), Val::Bit(Bit::from(common.const_)));
    }
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
