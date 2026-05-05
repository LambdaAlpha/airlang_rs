use std::ops::DerefMut;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::CtxFreeInputAwareFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::func::PrimCtx;
use crate::semantics::val::LINK;
use crate::semantics::val::LinkVal;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Key;
use crate::type_::Pair;

// todo design
#[derive(Copy, Clone)]
pub struct LinkLib {
    pub make: PrimFuncVal,
    pub make_constant: PrimFuncVal,
    pub is_constant: PrimFuncVal,
    pub is_available: PrimFuncVal,
    pub get_id: PrimFuncVal,
    pub let_: PrimFuncVal,
}

pub const MAKE: &str = concatcp!(PREFIX_CELL, LINK, ".make");
pub const MAKE_CONSTANT: &str = concatcp!(PREFIX_CELL, LINK, ".make_constant");
pub const IS_CONSTANT: &str = concatcp!(PREFIX_CELL, LINK, ".is_constant");
pub const IS_AVAILABLE: &str = concatcp!(PREFIX_CELL, LINK, ".is_available");
pub const GET_ID: &str = concatcp!(PREFIX_CELL, LINK, ".get_id");
pub const LET: &str = concatcp!(PREFIX_CELL, LINK, ".let");

impl Default for LinkLib {
    fn default() -> Self {
        Self {
            make: CtxFreeInputAwareFunc { fn_: make }.build(),
            make_constant: CtxFreeInputAwareFunc { fn_: make_constant }.build(),
            is_constant: CtxFreeInputAwareFunc { fn_: is_constant }.build(),
            is_available: CtxFreeInputAwareFunc { fn_: is_available }.build(),
            get_id: CtxFreeInputAwareFunc { fn_: get_id }.build(),
            let_: CtxFreeInputAwareFunc { fn_: let_ }.build(),
        }
    }
}

impl CfgMod for LinkLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, MAKE, self.make);
        extend_func(cfg, MAKE_CONSTANT, self.make_constant);
        extend_func(cfg, IS_CONSTANT, self.is_constant);
        extend_func(cfg, IS_AVAILABLE, self.is_available);
        extend_func(cfg, GET_ID, self.get_id);
        extend_func(cfg, LET, self.let_);
    }
}

pub fn make(_cfg: &mut Cfg, input: Val) -> Val {
    Val::Link(LinkVal::new(input, false))
}

pub fn make_constant(_cfg: &mut Cfg, input: Val) -> Val {
    Val::Link(LinkVal::new(input, true))
}

pub fn is_constant(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Link(link) = input else {
        return bug!(cfg, "{IS_CONSTANT}: expected input to be a link, but got {input}");
    };
    Val::Bit(Bit::from(link.is_const()))
}

pub fn is_available(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Link(link) = input else {
        return bug!(cfg, "{IS_AVAILABLE}: expected input to be a link, but got {input}");
    };
    let available = link.try_borrow().is_ok();
    Val::Bit(Bit::from(available))
}

pub fn get_id(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Link(link) = input else {
        return bug!(cfg, "{GET_ID}: expected input to be a link, but got {input}");
    };
    let id = link.ptr_addr();
    let id = Key::from_string_unchecked(format!("{id:x}"));
    Val::Key(id)
}

pub fn let_(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{LET}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Link(link) = pair.left else {
        return bug!(cfg, "{LET}: expected input.left to be a link, but got {}", pair.left);
    };
    let Val::Pair(func_input) = pair.right else {
        return bug!(cfg, "{LET}: expected input.right to be a pair, but got {}", pair.right);
    };
    let func_input = Pair::from(func_input);
    let Val::Func(func) = func_input.left else {
        return bug!(cfg, "{LET}: expected input.right.left to be a function, \
            but got {}", func_input.left);
    };
    // todo design support control flow
    if link.is_const() && matches!(func.ctx(), PrimCtx::Mut) {
        return bug!(cfg, "{LET}: expected input.right.left to be a context-constant function, \
            but got {func}");
    }
    let Ok(mut ctx) = link.try_borrow_mut() else {
        return bug!(cfg, "{LET}: link is not available");
    };
    func.call(cfg, ctx.deref_mut(), func_input.right)
}
