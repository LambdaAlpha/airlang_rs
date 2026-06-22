use std::mem::swap;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::export_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::ConstInputFreeFunc;
use crate::semantics::func::CtxFreeFunc;
use crate::semantics::func::MutFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::QUOTE;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::Quote;

#[derive(Copy, Clone)]
pub struct QuoteLib {
    pub make: PrimFuncVal,
    pub get_value: PrimFuncVal,
    pub set_value: PrimFuncVal,
}

pub const MAKE: &str = concatcp!(PREFIX_CELL, QUOTE, ".make");
pub const GET_VALUE: &str = concatcp!(PREFIX_CELL, QUOTE, ".get_value");
pub const SET_VALUE: &str = concatcp!(PREFIX_CELL, QUOTE, ".set_value");

impl Default for QuoteLib {
    fn default() -> Self {
        Self {
            make: CtxFreeFunc { fn_: make }.build(),
            get_value: ConstInputFreeFunc { fn_: get_value }.build(),
            set_value: MutFunc { fn_: set_value }.build(),
        }
    }
}

impl CfgMod for QuoteLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        export_func(cfg, MAKE, self.make);
        export_func(cfg, GET_VALUE, self.get_value);
        export_func(cfg, SET_VALUE, self.set_value);
    }
}

pub fn make(_cfg: &mut Cfg, input: Val) -> Val {
    Val::Quote(Quote::new(input).into())
}

pub fn get_value(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Quote(quote) = ctx else {
        return bug!(cfg, "{GET_VALUE}: expected context to be a quote, but got {ctx}");
    };
    quote.value.clone()
}

pub fn set_value(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Quote(quote) = ctx else {
        return bug!(cfg, "{SET_VALUE}: expected context to be a quote, but got {ctx}");
    };
    swap(&mut quote.value, &mut input);
    input
}
