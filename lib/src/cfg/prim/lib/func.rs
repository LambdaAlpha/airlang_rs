use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::cfg::repr::func::generate_code;
use crate::cfg::repr::func::generate_func;
use crate::cfg::repr::func::parse_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxFreeInputAwareFunc;
use crate::semantics::func::PrimCtx;
use crate::semantics::func::PrimInput;
use crate::semantics::val::FUNC;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;

#[derive(Clone)]
pub struct FuncLib {
    pub make: PrimFuncVal,
    pub represent: PrimFuncVal,
    pub is_free: PrimFuncVal,
    pub is_constant: PrimFuncVal,
    pub is_input_free: PrimFuncVal,
    pub is_primitive: PrimFuncVal,
    pub get_code: PrimFuncVal,
    pub get_prelude: PrimFuncVal,
}

pub const MAKE: &str = concatcp!(PREFIX_CELL, FUNC, ".make");
pub const REPRESENT: &str = concatcp!(PREFIX_CELL, FUNC, ".represent");
pub const IS_FREE: &str = concatcp!(PREFIX_CELL, FUNC, ".is_free");
pub const IS_CONSTANT: &str = concatcp!(PREFIX_CELL, FUNC, ".is_constant");
pub const IS_INPUT_FREE: &str = concatcp!(PREFIX_CELL, FUNC, ".is_input_free");
pub const IS_PRIMITIVE: &str = concatcp!(PREFIX_CELL, FUNC, ".is_primitive");
pub const GET_CODE: &str = concatcp!(PREFIX_CELL, FUNC, ".get_code");
pub const GET_PRELUDE: &str = concatcp!(PREFIX_CELL, FUNC, ".get_prelude");

impl Default for FuncLib {
    fn default() -> Self {
        FuncLib {
            make: CtxFreeInputAwareFunc { fn_: make }.build(),
            represent: CtxFreeInputAwareFunc { fn_: represent }.build(),
            is_free: CtxConstInputFreeFunc { fn_: is_free }.build(),
            is_constant: CtxConstInputFreeFunc { fn_: is_constant }.build(),
            is_input_free: CtxConstInputFreeFunc { fn_: is_input_free }.build(),
            is_primitive: CtxConstInputFreeFunc { fn_: is_primitive }.build(),
            get_code: CtxConstInputFreeFunc { fn_: get_code }.build(),
            get_prelude: CtxConstInputFreeFunc { fn_: get_prelude }.build(),
        }
    }
}

impl CfgMod for FuncLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, MAKE, self.make);
        extend_func(cfg, REPRESENT, self.represent);
        extend_func(cfg, IS_FREE, self.is_free);
        extend_func(cfg, IS_CONSTANT, self.is_constant);
        extend_func(cfg, IS_INPUT_FREE, self.is_input_free);
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
    Val::Map(generate_func(func))
}

pub fn is_free(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{IS_FREE}: expected context to be a function, but got {ctx}");
    };
    Val::Bit(Bit::from(matches!(func.ctx(), PrimCtx::Free)))
}

pub fn is_constant(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Func(func) = ctx else {
        return bug!(cfg, "{IS_CONSTANT}: expected context to be a function, \
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
