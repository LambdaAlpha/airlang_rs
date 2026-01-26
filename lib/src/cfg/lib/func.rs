use std::ops::Deref;
use std::rc::Rc;

use const_format::concatcp;

use self::repr::generate_func;
use self::repr::parse_func;
use super::ConstImpl;
use super::FreeImpl;
use super::abort_free;
use super::func::repr::generate_code;
use super::func::repr::generate_ctx_access;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::func::MutPrimFunc;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FUNC;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Key;
use crate::type_::Pair;

#[derive(Clone)]
pub struct FuncLib {
    pub new: FreePrimFuncVal,
    pub represent: FreePrimFuncVal,
    pub apply: MutPrimFuncVal,
    pub get_context_access: ConstPrimFuncVal,
    pub is_raw_input: ConstPrimFuncVal,
    pub is_primitive: ConstPrimFuncVal,
    pub get_code: ConstPrimFuncVal,
    pub get_prelude: ConstPrimFuncVal,
}

pub const NEW: &str = concatcp!(PREFIX_ID, FUNC, ".new");
pub const REPRESENT: &str = concatcp!(PREFIX_ID, FUNC, ".represent");
pub const APPLY: &str = concatcp!(PREFIX_ID, FUNC, ".apply");
pub const GET_CONTEXT_ACCESS: &str = concatcp!(PREFIX_ID, FUNC, ".get_context_access");
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
            get_context_access: get_context_access(),
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
        extend_func(cfg, GET_CONTEXT_ACCESS, self.get_context_access);
        extend_func(cfg, IS_RAW_INPUT, self.is_raw_input);
        extend_func(cfg, IS_PRIMITIVE, self.is_primitive);
        extend_func(cfg, GET_CODE, self.get_code);
        extend_func(cfg, GET_PRELUDE, self.get_prelude);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreeImpl { free: fn_new }.build()
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Some(func) = parse_func(cfg, input) else {
        return Val::default();
    };
    Val::Func(func)
}

pub fn represent() -> FreePrimFuncVal {
    FreeImpl { free: fn_represent }.build()
}

fn fn_represent(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        return bug!(cfg, "{REPRESENT}: expected input to be a function, but got {input:?}");
    };
    generate_func(func)
}

pub fn apply() -> MutPrimFuncVal {
    MutPrimFunc { raw_input: false, fn_: Rc::new(Apply) }.into()
}

struct Apply;

impl FreeFn<Cfg, Val, Val> for Apply {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        let (func, input) = match func_input(cfg, input) {
            Ok(pair) => pair,
            Err(err) => return err,
        };
        func.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Apply {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        let (func, input) = match func_input(cfg, input) {
            Ok(pair) => pair,
            Err(err) => return err,
        };
        func.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, Val, Val> for Apply {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        let (func, input) = match func_input(cfg, input) {
            Ok(pair) => pair,
            Err(err) => return err,
        };
        func.mut_call(cfg, ctx, input)
    }
}

fn func_input(cfg: &mut Cfg, input: Val) -> Result<(FuncVal, Val), Val> {
    let Val::Pair(pair) = input else {
        return Err(bug!(cfg, "{APPLY}: expected input to be a pair, but got {input:?}"));
    };
    let pair = Pair::from(pair);
    let Val::Func(func) = pair.left else {
        return Err(bug!(
            cfg,
            "{APPLY}: expected input.left to be a function, but got {:?}",
            pair.left
        ));
    };
    Ok((func, pair.right))
}

pub fn get_context_access() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_CONTEXT_ACCESS), const_: fn_get_context_access }.build()
}

fn fn_get_context_access(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return bug!(
            cfg,
            "{GET_CONTEXT_ACCESS}: expected context to be a function, but got {:?}",
            ctx.deref()
        );
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_CONTEXT_ACCESS}: expected input to be a unit, but got {input:?}");
    }
    let access = generate_ctx_access(func.ctx_access());
    Val::Key(Key::from_str_unchecked(access))
}

pub fn is_raw_input() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(IS_RAW_INPUT), const_: fn_is_raw_input }.build()
}

fn fn_is_raw_input(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return bug!(
            cfg,
            "{IS_RAW_INPUT}: expected context to be a function, but got {:?}",
            ctx.deref()
        );
    };
    if !input.is_unit() {
        return bug!(cfg, "{IS_RAW_INPUT}: expected input to be a unit, but got {input:?}");
    }
    Val::Bit(Bit::from(func.raw_input()))
}

pub fn is_primitive() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(IS_PRIMITIVE), const_: fn_is_primitive }.build()
}

fn fn_is_primitive(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return bug!(
            cfg,
            "{IS_PRIMITIVE}: expected context to be a function, but got {:?}",
            ctx.deref()
        );
    };
    if !input.is_unit() {
        return bug!(cfg, "{IS_PRIMITIVE}: expected input to be a unit, but got {input:?}");
    }
    let is_primitive = func.is_primitive();
    Val::Bit(Bit::from(is_primitive))
}

pub fn get_code() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_CODE), const_: fn_get_code }.build()
}

fn fn_get_code(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return bug!(
            cfg,
            "{GET_CODE}: expected context to be a function, but got {:?}",
            ctx.deref()
        );
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_CODE}: expected input to be a unit, but got {input:?}");
    }
    generate_code(func)
}

pub fn get_prelude() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_PRELUDE), const_: fn_get_prelude }.build()
}

fn fn_get_prelude(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        return bug!(
            cfg,
            "{GET_PRELUDE}: expected context to be a function, but got {:?}",
            ctx.deref()
        );
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_PRELUDE}: expected input to be a unit, but got {input:?}");
    }
    let Some(ctx) = func.prelude() else {
        return bug!(cfg, "{GET_PRELUDE}: prelude not found");
    };
    ctx.clone()
}

mod repr;
