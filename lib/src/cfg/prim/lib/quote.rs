use std::mem::swap;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxFreeInputAwareFunc;
use crate::semantics::func::CtxMutInputAwareFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::QUOTE;
use crate::semantics::val::Val;
use crate::type_::Quote;

#[derive(Copy, Clone)]
pub struct QuoteLib {
    pub make: PrimFuncVal,
    pub get_source: PrimFuncVal,
    pub set_source: PrimFuncVal,
}

pub const MAKE: &str = concatcp!(PREFIX_CELL, QUOTE, ".make");
pub const GET_SOURCE: &str = concatcp!(PREFIX_CELL, QUOTE, ".get_source");
pub const SET_SOURCE: &str = concatcp!(PREFIX_CELL, QUOTE, ".set_source");

impl Default for QuoteLib {
    fn default() -> Self {
        Self {
            make: CtxFreeInputAwareFunc { fn_: make }.build(),
            get_source: CtxConstInputFreeFunc { fn_: get_source }.build(),
            set_source: CtxMutInputAwareFunc { fn_: set_source }.build(),
        }
    }
}

impl CfgMod for QuoteLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, MAKE, self.make);
        extend_func(cfg, GET_SOURCE, self.get_source);
        extend_func(cfg, SET_SOURCE, self.set_source);
    }
}

pub fn make(_cfg: &mut Cfg, input: Val) -> Val {
    Val::Quote(Quote::new(input).into())
}

pub fn get_source(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Quote(quote) = ctx else {
        return bug!(cfg, "{GET_SOURCE}: expected context to be a quote, but got {ctx}");
    };
    quote.source.clone()
}

pub fn set_source(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Quote(quote) = ctx else {
        return bug!(cfg, "{SET_SOURCE}: expected context to be a quote, but got {ctx}");
    };
    swap(&mut quote.source, &mut input);
    input
}
