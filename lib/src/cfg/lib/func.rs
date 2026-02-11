use std::rc::Rc;

use const_format::concatcp;

use super::ConstImpl;
use super::FreeImpl;
use super::ImplExtra;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::cfg::repr::func::generate_code;
use crate::cfg::repr::func::generate_func;
use crate::cfg::repr::func::parse_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::DynFunc;
use crate::semantics::func::PrimCtx;
use crate::semantics::func::PrimFunc;
use crate::semantics::val::FUNC;
use crate::semantics::val::FuncVal;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Pair;

#[derive(Clone)]
pub struct FuncLib {
    pub new: PrimFuncVal,
    pub represent: PrimFuncVal,
    pub apply: PrimFuncVal,
    pub is_context_free: PrimFuncVal,
    pub is_context_constant: PrimFuncVal,
    pub is_raw_input: PrimFuncVal,
    pub is_primitive: PrimFuncVal,
    pub get_code: PrimFuncVal,
    pub get_prelude: PrimFuncVal,
}

pub const NEW: &str = concatcp!(PREFIX_ID, FUNC, ".new");
pub const REPRESENT: &str = concatcp!(PREFIX_ID, FUNC, ".represent");
pub const APPLY: &str = concatcp!(PREFIX_ID, FUNC, ".apply");
pub const IS_CONTEXT_FREE: &str = concatcp!(PREFIX_ID, FUNC, ".is_context_free");
pub const IS_CONTEXT_CONSTANT: &str = concatcp!(PREFIX_ID, FUNC, ".is_context_constant");
pub const IS_RAW_INPUT: &str = concatcp!(PREFIX_ID, FUNC, ".is_raw_input");
pub const IS_PRIMITIVE: &str = concatcp!(PREFIX_ID, FUNC, ".is_primitive");
pub const GET_CODE: &str = concatcp!(PREFIX_ID, FUNC, ".get_code");
pub const GET_PRELUDE: &str = concatcp!(PREFIX_ID, FUNC, ".get_prelude");

impl Default for FuncLib {
    fn default() -> Self {
        FuncLib {
            new: new(),
            represent: represent(),
            apply: apply(),
            is_context_free: is_context_free(),
            is_context_constant: is_context_constant(),
            is_raw_input: is_raw_input(),
            is_primitive: is_primitive(),
            get_code: get_code(),
            get_prelude: get_prelude(),
        }
    }
}

impl CfgMod for FuncLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, NEW, self.new);
        extend_func(cfg, REPRESENT, self.represent);
        extend_func(cfg, APPLY, self.apply);
        extend_func(cfg, IS_CONTEXT_FREE, self.is_context_free);
        extend_func(cfg, IS_CONTEXT_CONSTANT, self.is_context_constant);
        extend_func(cfg, IS_RAW_INPUT, self.is_raw_input);
        extend_func(cfg, IS_PRIMITIVE, self.is_primitive);
        extend_func(cfg, GET_CODE, self.get_code);
        extend_func(cfg, GET_PRELUDE, self.get_prelude);
    }
}

pub fn new() -> PrimFuncVal {
    FreeImpl { fn_: fn_new }.build(ImplExtra { raw_input: false })
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Some(func) = parse_func(cfg, input) else {
        return Val::default();
    };
    Val::Func(func)
}

pub fn represent() -> PrimFuncVal {
    FreeImpl { fn_: fn_represent }.build(ImplExtra { raw_input: false })
}

fn fn_represent(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        return bug!(cfg, "{REPRESENT}: expected input to be a function, but got {input}");
    };
    generate_func(func)
}

pub fn apply() -> PrimFuncVal {
    PrimFunc { raw_input: false, fn_: Rc::new(Apply), ctx: PrimCtx::Mut }.into()
}

struct Apply;

impl DynFunc<Cfg, Val, Val, Val> for Apply {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        let (func, input) = match func_input(cfg, input) {
            Ok(pair) => pair,
            Err(err) => return err,
        };
        func.call(cfg, ctx, input)
    }
}

fn func_input(cfg: &mut Cfg, input: Val) -> Result<(FuncVal, Val), Val> {
    let Val::Pair(pair) = input else {
        return Err(bug!(cfg, "{APPLY}: expected input to be a pair, but got {input}"));
    };
    let pair = Pair::from(pair);
    let Val::Func(func) = pair.left else {
        return Err(bug!(
            cfg,
            "{APPLY}: expected input.left to be a function, but got {}",
            pair.left
        ));
    };
    Ok((func, pair.right))
}

pub fn is_context_free() -> PrimFuncVal {
    ConstImpl { fn_: fn_is_context_free }.build(ImplExtra { raw_input: false })
}

fn fn_is_context_free(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{IS_CONTEXT_FREE}: expected context to be a function, but got {ctx}");
    };
    if !input.is_unit() {
        return bug!(cfg, "{IS_CONTEXT_FREE}: expected input to be a unit, but got {input}");
    }
    Val::Bit(Bit::from(func.is_free()))
}

pub fn is_context_constant() -> PrimFuncVal {
    ConstImpl { fn_: fn_is_context_constant }.build(ImplExtra { raw_input: false })
}

fn fn_is_context_constant(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(
            cfg,
            "{IS_CONTEXT_CONSTANT}: expected context to be a function, but got {ctx}"
        );
    };
    if !input.is_unit() {
        return bug!(cfg, "{IS_CONTEXT_CONSTANT}: expected input to be a unit, but got {input}");
    }
    Val::Bit(Bit::from(func.is_const()))
}

pub fn is_raw_input() -> PrimFuncVal {
    ConstImpl { fn_: fn_is_raw_input }.build(ImplExtra { raw_input: false })
}

fn fn_is_raw_input(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{IS_RAW_INPUT}: expected context to be a function, but got {ctx}");
    };
    if !input.is_unit() {
        return bug!(cfg, "{IS_RAW_INPUT}: expected input to be a unit, but got {input}");
    }
    Val::Bit(Bit::from(func.raw_input()))
}

pub fn is_primitive() -> PrimFuncVal {
    ConstImpl { fn_: fn_is_primitive }.build(ImplExtra { raw_input: false })
}

fn fn_is_primitive(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{IS_PRIMITIVE}: expected context to be a function, but got {ctx}");
    };
    if !input.is_unit() {
        return bug!(cfg, "{IS_PRIMITIVE}: expected input to be a unit, but got {input}");
    }
    let is_primitive = func.is_primitive();
    Val::Bit(Bit::from(is_primitive))
}

pub fn get_code() -> PrimFuncVal {
    ConstImpl { fn_: fn_get_code }.build(ImplExtra { raw_input: false })
}

fn fn_get_code(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{GET_CODE}: expected context to be a function, but got {ctx}");
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_CODE}: expected input to be a unit, but got {input}");
    }
    generate_code(func)
}

pub fn get_prelude() -> PrimFuncVal {
    ConstImpl { fn_: fn_get_prelude }.build(ImplExtra { raw_input: false })
}

fn fn_get_prelude(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{GET_PRELUDE}: expected context to be a function, but got {ctx}");
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_PRELUDE}: expected input to be a unit, but got {input}");
    }
    let Some(ctx) = func.prelude() else {
        return bug!(cfg, "{GET_PRELUDE}: prelude not found");
    };
    ctx.clone()
}
