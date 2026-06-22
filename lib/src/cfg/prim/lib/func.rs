use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::export_func;
use crate::cfg::repr::func::generate_code;
use crate::cfg::repr::func::generate_func;
use crate::cfg::repr::func::parse_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::CtxFreeFunc;
use crate::semantics::func::PrimCtx;
use crate::semantics::func::PrimInput;
use crate::semantics::val::FUNC;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Key;
use crate::type_::Map;

#[derive(Copy, Clone)]
pub struct FuncLib {
    pub make: PrimFuncVal,
    pub represent: PrimFuncVal,
    pub is_free: PrimFuncVal,
    pub is_constant: PrimFuncVal,
    pub is_input_free: PrimFuncVal,
    pub is_primitive: PrimFuncVal,
    pub get_code: PrimFuncVal,
    pub get_prelude: PrimFuncVal,
    pub get_id: PrimFuncVal,
}

pub const MAKE: &str = concatcp!(PREFIX_CELL, FUNC, ".make");
pub const REPRESENT: &str = concatcp!(PREFIX_CELL, FUNC, ".represent");
pub const IS_FREE: &str = concatcp!(PREFIX_CELL, FUNC, ".is_free");
pub const IS_CONSTANT: &str = concatcp!(PREFIX_CELL, FUNC, ".is_constant");
pub const IS_INPUT_FREE: &str = concatcp!(PREFIX_CELL, FUNC, ".is_input_free");
pub const IS_PRIMITIVE: &str = concatcp!(PREFIX_CELL, FUNC, ".is_primitive");
pub const GET_CODE: &str = concatcp!(PREFIX_CELL, FUNC, ".get_code");
pub const GET_PRELUDE: &str = concatcp!(PREFIX_CELL, FUNC, ".get_prelude");
pub const GET_ID: &str = concatcp!(PREFIX_CELL, FUNC, ".get_id");

impl Default for FuncLib {
    fn default() -> Self {
        Self {
            make: CtxFreeFunc { fn_: make }.build(),
            represent: CtxFreeFunc { fn_: represent }.build(),
            is_free: CtxFreeFunc { fn_: is_free }.build(),
            is_constant: CtxFreeFunc { fn_: is_constant }.build(),
            is_input_free: CtxFreeFunc { fn_: is_input_free }.build(),
            is_primitive: CtxFreeFunc { fn_: is_primitive }.build(),
            get_code: CtxFreeFunc { fn_: get_code }.build(),
            get_prelude: CtxFreeFunc { fn_: get_prelude }.build(),
            get_id: CtxFreeFunc { fn_: get_id }.build(),
        }
    }
}

impl CfgMod for FuncLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        export_func(cfg, MAKE, self.make);
        export_func(cfg, REPRESENT, self.represent);
        export_func(cfg, IS_FREE, self.is_free);
        export_func(cfg, IS_CONSTANT, self.is_constant);
        export_func(cfg, IS_INPUT_FREE, self.is_input_free);
        export_func(cfg, IS_PRIMITIVE, self.is_primitive);
        export_func(cfg, GET_CODE, self.get_code);
        export_func(cfg, GET_PRELUDE, self.get_prelude);
        export_func(cfg, GET_ID, self.get_id);
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

pub fn is_free(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        return bug!(cfg, "{IS_FREE}: expected input to be a function, but got {input}");
    };
    Val::Bit(Bit::from(matches!(func.ctx(), PrimCtx::Free)))
}

pub fn is_constant(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        return bug!(cfg, "{IS_CONSTANT}: expected input to be a function, but got {input}");
    };
    match func.ctx() {
        PrimCtx::Free => Val::Bit(Bit::from(true)),
        PrimCtx::Const_ => Val::Bit(Bit::from(true)),
        PrimCtx::Mut => Val::Bit(Bit::from(false)),
        PrimCtx::Default => Val::default(),
    }
}

pub fn is_input_free(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        return bug!(cfg, "{IS_INPUT_FREE}: expected input to be a function, but got {input}");
    };
    Val::Bit(Bit::from(matches!(func.input(), PrimInput::Free)))
}

pub fn is_primitive(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        return bug!(cfg, "{IS_PRIMITIVE}: expected input to be a function, but got {input}");
    };
    let is_primitive = func.is_primitive();
    Val::Bit(Bit::from(is_primitive))
}

pub fn get_code(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        return bug!(cfg, "{GET_CODE}: expected input to be a function, but got {input}");
    };
    generate_code(&func)
}

pub fn get_prelude(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        return bug!(cfg, "{GET_PRELUDE}: expected input to be a function, but got {input}");
    };
    let Some(ctx) = func.prelude() else {
        return bug!(cfg, "{GET_PRELUDE}: prelude not found");
    };
    ctx.clone()
}

pub fn get_id(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        return bug!(cfg, "{GET_ID}: expected input to be a function, but got {input}");
    };
    let id = func.id();
    let id = Key::from_string_unchecked(format!("{id:x}"));
    Val::Key(id)
}
