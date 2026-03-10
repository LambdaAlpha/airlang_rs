use std::mem::swap;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxMutInputAwareFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::QUOTE;
use crate::semantics::val::Val;

#[derive(Clone)]
pub struct QuoteLib {
    pub get_source: PrimFuncVal,
    pub set_source: PrimFuncVal,
}

pub const GET_SOURCE: &str = concatcp!(PREFIX_ID, QUOTE, ".get_source");
pub const SET_SOURCE: &str = concatcp!(PREFIX_ID, QUOTE, ".set_source");

impl Default for QuoteLib {
    fn default() -> Self {
        QuoteLib {
            get_source: CtxConstInputFreeFunc { fn_: get_source }.build(),
            set_source: CtxMutInputAwareFunc { fn_: set_source }.build(),
        }
    }
}

impl CfgMod for QuoteLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, GET_SOURCE, self.get_source);
        extend_func(cfg, SET_SOURCE, self.set_source);
    }
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
