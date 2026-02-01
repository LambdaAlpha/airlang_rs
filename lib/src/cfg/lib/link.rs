use std::ops::DerefMut;

use const_format::concatcp;

use super::FreeImpl;
use super::ImplExtra;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxFn;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::LINK;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Pair;

// todo design
#[derive(Clone)]
pub struct LinkLib {
    pub new: FreePrimFuncVal,
    pub new_constant: FreePrimFuncVal,
    pub is_constant: FreePrimFuncVal,
    pub which: FreePrimFuncVal,
}

pub const NEW: &str = concatcp!(PREFIX_ID, LINK, ".new");
pub const NEW_CONSTANT: &str = concatcp!(PREFIX_ID, LINK, ".new_constant");
pub const IS_CONSTANT: &str = concatcp!(PREFIX_ID, LINK, ".is_constant");
pub const WHICH: &str = concatcp!(PREFIX_ID, LINK, ".which");

impl Default for LinkLib {
    fn default() -> Self {
        LinkLib {
            new: new(),
            new_constant: new_constant(),
            is_constant: is_constant(),
            which: which(),
        }
    }
}

impl CfgMod for LinkLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, NEW, self.new);
        extend_func(cfg, NEW_CONSTANT, self.new_constant);
        extend_func(cfg, IS_CONSTANT, self.is_constant);
        extend_func(cfg, WHICH, self.which);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreeImpl { fn_: fn_new }.build(ImplExtra { raw_input: false })
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    Val::Link(LinkVal::new(input, false))
}

pub fn new_constant() -> FreePrimFuncVal {
    FreeImpl { fn_: fn_new_constant }.build(ImplExtra { raw_input: false })
}

fn fn_new_constant(_cfg: &mut Cfg, input: Val) -> Val {
    Val::Link(LinkVal::new(input, true))
}

pub fn is_constant() -> FreePrimFuncVal {
    FreeImpl { fn_: fn_is_constant }.build(ImplExtra { raw_input: false })
}

fn fn_is_constant(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Link(link) = input else {
        return bug!(cfg, "{WHICH}: expected input to be a link, but got {input}");
    };
    Val::Bit(Bit::from(link.is_const()))
}

pub fn which() -> FreePrimFuncVal {
    FreeImpl { fn_: fn_which }.build(ImplExtra { raw_input: false })
}

fn fn_which(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{WHICH}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Link(link) = pair.left else {
        return bug!(cfg, "{WHICH}: expected input.left to be a link, but got {}", pair.left);
    };
    let Val::Pair(func_input) = pair.right else {
        return bug!(cfg, "{WHICH}: expected input.right to be a pair, but got {}", pair.right);
    };
    let func_input = Pair::from(func_input);
    let Val::Func(func) = func_input.left else {
        return bug!(
            cfg,
            "{WHICH}: expected input.right.left to be a function, but got {}",
            func_input.left
        );
    };
    // todo design support control flow
    if link.is_const() && !func.is_const() {
        return bug!(
            cfg,
            "{WHICH}: expected input.right.left to be a context-constant function, but got {func}"
        );
    }
    let Ok(mut ctx) = link.try_borrow_mut() else {
        return bug!(cfg, "{WHICH}: link is in use");
    };
    func.ctx_call(cfg, ctx.deref_mut(), func_input.right)
}
