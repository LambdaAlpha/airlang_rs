use std::rc::Rc;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::cfg::repr::func::generate_code;
use crate::cfg::repr::func::generate_func;
use crate::cfg::repr::func::parse_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxFreeInputEvalFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::func::PrimCtx;
use crate::semantics::func::PrimFunc;
use crate::semantics::func::PrimInput;
use crate::semantics::val::FUNC;
use crate::semantics::val::FuncVal;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Pair;

#[derive(Clone)]
pub struct FuncLib {
    pub make: PrimFuncVal,
    pub represent: PrimFuncVal,
    pub apply: PrimFuncVal,
    pub is_context_free: PrimFuncVal,
    pub is_context_constant: PrimFuncVal,
    pub is_input_free: PrimFuncVal,
    pub is_input_raw: PrimFuncVal,
    pub is_primitive: PrimFuncVal,
    pub get_code: PrimFuncVal,
    pub get_prelude: PrimFuncVal,
}

pub const MAKE: &str = concatcp!(PREFIX_ID, FUNC, ".make");
pub const REPRESENT: &str = concatcp!(PREFIX_ID, FUNC, ".represent");
pub const APPLY: &str = concatcp!(PREFIX_ID, FUNC, ".apply");
pub const IS_CONTEXT_FREE: &str = concatcp!(PREFIX_ID, FUNC, ".is_context_free");
pub const IS_CONTEXT_CONSTANT: &str = concatcp!(PREFIX_ID, FUNC, ".is_context_constant");
pub const IS_INPUT_FREE: &str = concatcp!(PREFIX_ID, FUNC, ".is_input_free");
pub const IS_INPUT_RAW: &str = concatcp!(PREFIX_ID, FUNC, ".is_input_raw");
pub const IS_PRIMITIVE: &str = concatcp!(PREFIX_ID, FUNC, ".is_primitive");
pub const GET_CODE: &str = concatcp!(PREFIX_ID, FUNC, ".get_code");
pub const GET_PRELUDE: &str = concatcp!(PREFIX_ID, FUNC, ".get_prelude");

impl Default for FuncLib {
    fn default() -> Self {
        FuncLib {
            make: CtxFreeInputEvalFunc { fn_: make }.build(),
            represent: CtxFreeInputEvalFunc { fn_: represent }.build(),
            apply: PrimFunc { fn_: Rc::new(Apply), ctx: PrimCtx::Mut, input: PrimInput::Eval }
                .into(),
            is_context_free: CtxConstInputFreeFunc { fn_: is_context_free }.build(),
            is_context_constant: CtxConstInputFreeFunc { fn_: is_context_constant }.build(),
            is_input_free: CtxConstInputFreeFunc { fn_: is_input_free }.build(),
            is_input_raw: CtxConstInputFreeFunc { fn_: is_input_raw }.build(),
            is_primitive: CtxConstInputFreeFunc { fn_: is_primitive }.build(),
            get_code: CtxConstInputFreeFunc { fn_: get_code }.build(),
            get_prelude: CtxConstInputFreeFunc { fn_: get_prelude }.build(),
        }
    }
}

impl CfgMod for FuncLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, MAKE, self.make);
        extend_func(cfg, REPRESENT, self.represent);
        extend_func(cfg, APPLY, self.apply);
        extend_func(cfg, IS_CONTEXT_FREE, self.is_context_free);
        extend_func(cfg, IS_CONTEXT_CONSTANT, self.is_context_constant);
        extend_func(cfg, IS_INPUT_FREE, self.is_input_free);
        extend_func(cfg, IS_INPUT_RAW, self.is_input_raw);
        extend_func(cfg, IS_PRIMITIVE, self.is_primitive);
        extend_func(cfg, GET_CODE, self.get_code);
        extend_func(cfg, GET_PRELUDE, self.get_prelude);
    }
}

pub fn make(cfg: &mut Cfg, input: Val) -> Val {
    let Some(func) = parse_func(cfg, input) else {
        return Val::default();
    };
    Val::Func(func)
}

pub fn represent(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        return bug!(cfg, "{REPRESENT}: expected input to be a function, but got {input}");
    };
    generate_func(func)
}

pub struct Apply;

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
        return Err(bug!(cfg, "{APPLY}: expected input.left to be a function, \
            but got {}", pair.left));
    };
    Ok((func, pair.right))
}

pub fn is_context_free(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{IS_CONTEXT_FREE}: expected context to be a function, but got {ctx}");
    };
    Val::Bit(Bit::from(matches!(func.ctx(), PrimCtx::Free)))
}

pub fn is_context_constant(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{IS_CONTEXT_CONSTANT}: expected context to be a function, \
            but got {ctx}");
    };
    Val::Bit(Bit::from(!matches!(func.ctx(), PrimCtx::Mut)))
}

pub fn is_input_free(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{IS_INPUT_FREE}: expected context to be a function, but got {ctx}");
    };
    Val::Bit(Bit::from(matches!(func.input(), PrimInput::Free)))
}

pub fn is_input_raw(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{IS_INPUT_RAW}: expected context to be a function, but got {ctx}");
    };
    Val::Bit(Bit::from(!matches!(func.input(), PrimInput::Eval)))
}

pub fn is_primitive(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{IS_PRIMITIVE}: expected context to be a function, but got {ctx}");
    };
    let is_primitive = func.is_primitive();
    Val::Bit(Bit::from(is_primitive))
}

pub fn get_code(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{GET_CODE}: expected context to be a function, but got {ctx}");
    };
    generate_code(func)
}

pub fn get_prelude(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{GET_PRELUDE}: expected context to be a function, but got {ctx}");
    };
    let Some(ctx) = func.prelude() else {
        return bug!(cfg, "{GET_PRELUDE}: prelude not found");
    };
    ctx.clone()
}
