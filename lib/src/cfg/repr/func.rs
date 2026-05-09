use crate::bug;
use crate::cfg::prim::lib::func::MAKE;
use crate::cfg::utils::map_remove;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::CompCtx;
use crate::semantics::func::CompFunc;
use crate::semantics::func::CompInput;
use crate::semantics::func::DynFunc;
use crate::semantics::func::PrimCtx;
use crate::semantics::func::PrimInput;
use crate::semantics::val::CompFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::Pair;

// todo rename
const CODE: &str = "code";
const PRELUDE: &str = "prelude";
const CONST: &str = "constant";

pub(in crate::cfg) fn parse_func(cfg: &mut Cfg, input: Val) -> Option<FuncVal> {
    let Val::Map(mut map) = input else {
        bug!(cfg, "{MAKE}: expected input to be a map, but got {input}");
        return None;
    };
    let CompCode { ctx_name, input_name, body } = parse_code(cfg, map_remove(&mut map, CODE))?;
    let prelude = map_remove(&mut map, PRELUDE);
    let ctx = if let Some(name) = ctx_name {
        let prim = parse_const(cfg, map_remove(&mut map, CONST))?;
        CompCtx { name, prim }
    } else {
        CompCtx { name: Key::default(), prim: PrimCtx::Free }
    };
    let input = if let Some(name) = input_name {
        CompInput { name, prim: PrimInput::Default }
    } else {
        CompInput { name: Key::default(), prim: PrimInput::Free }
    };
    let func = CompFunc { prelude, body, ctx, input };
    let func = FuncVal::Comp(CompFuncVal::from(func));
    Some(func)
}

fn parse_const(cfg: &mut Cfg, val: Val) -> Option<PrimCtx> {
    match val {
        Val::Unit(_) => Some(PrimCtx::Default),
        Val::Bit(bit) => Some(if *bit { PrimCtx::Const_ } else { PrimCtx::Mut }),
        v => {
            bug!(cfg, "{MAKE}: expected {CONST} to be a unit or a bit, but got {v}");
            None
        },
    }
}

pub(in crate::cfg) fn generate_func(f: FuncVal) -> MapVal {
    match f {
        FuncVal::Prim(f) => generate_prim(f),
        FuncVal::Comp(f) => generate_comp(f),
    }
}

struct CompCode {
    ctx_name: Option<Key>,
    input_name: Option<Key>,
    body: Val,
}

fn parse_code(cfg: &mut Cfg, code: Val) -> Option<CompCode> {
    let Val::Pair(names_body) = code else {
        bug!(cfg, "{MAKE}: expected {CODE} to be a pair, but got {code}");
        return None;
    };
    let names_body = Pair::from(names_body);
    let ctx_input = names_body.left;
    let body = names_body.right;
    let Val::Pair(ctx_input) = ctx_input else {
        bug!(cfg, "{MAKE}: expected names to be a pair, but got {ctx_input}");
        return None;
    };
    let ctx = &ctx_input.left;
    let ctx_name = match ctx {
        Val::Key(ctx) => Some(ctx.clone()),
        Val::Unit(_) => None,
        _ => {
            bug!(cfg, "{MAKE}: expected context name to be a key or a unit, but got {ctx}");
            return None;
        },
    };
    let input = &ctx_input.right;
    let input_name = match input {
        Val::Key(input) => Some(input.clone()),
        Val::Unit(_) => None,
        _ => {
            bug!(cfg, "{MAKE}: expected input name to be a key or a unit, but got {input}");
            return None;
        },
    };
    Some(CompCode { ctx_name, input_name, body })
}

pub(in crate::cfg) fn generate_code(func: &FuncVal) -> Val {
    match func {
        FuncVal::Prim(f) => prim_code(&f.fn_),
        FuncVal::Comp(f) => comp_code(f),
    }
}

fn prim_code(fn_: *const dyn DynFunc<Cfg, Val, Val, Val>) -> Val {
    let s = format!("{:x}", fn_.addr());
    Val::Key(Key::from_string_unchecked(s))
}

fn comp_code(comp: &CompFunc) -> Val {
    let ctx = match &comp.ctx.prim {
        PrimCtx::Free => Val::default(),
        _ => Val::Key(comp.ctx.name.clone()),
    };
    let input = match &comp.input.prim {
        PrimInput::Free => Val::default(),
        PrimInput::Default => Val::Key(comp.input.name.clone()),
    };
    let names = Val::Pair(Pair::new(ctx, input).into());
    Val::Pair(Pair::new(names, comp.body.clone()).into())
}

fn generate_prim(f: PrimFuncVal) -> MapVal {
    prim(PrimRepr { common: CommonRepr { ctx: f.ctx, code: prim_code(&f.fn_) } })
}

fn generate_comp(f: CompFuncVal) -> MapVal {
    comp(CompRepr {
        common: CommonRepr { ctx: f.ctx.prim, code: comp_code(&f) },
        prelude: f.prelude.clone(),
    })
}

struct CommonRepr {
    ctx: PrimCtx,
    code: Val,
}

fn generate_common(repr: &mut Map<Key, Val>, common: CommonRepr) {
    repr.insert(Key::from_str_unchecked(CODE), common.code);
    let const_ = !matches!(common.ctx, PrimCtx::Default);
    repr.insert(Key::from_str_unchecked(CONST), Val::Bit(Bit::from(const_)));
}

struct PrimRepr {
    common: CommonRepr,
}

fn prim(prim: PrimRepr) -> MapVal {
    let mut repr = Map::<Key, Val>::default();
    generate_common(&mut repr, prim.common);
    repr.into()
}

struct CompRepr {
    common: CommonRepr,
    prelude: Val,
}

fn comp(comp: CompRepr) -> MapVal {
    let mut repr = Map::<Key, Val>::default();
    generate_common(&mut repr, comp.common);
    repr.insert(Key::from_str_unchecked(PRELUDE), comp.prelude);
    repr.into()
}
