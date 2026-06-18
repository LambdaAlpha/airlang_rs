use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::ctx::Ctx;
use crate::semantics::fact::Fact;
use crate::semantics::func::CtxFreeFunc;
use crate::semantics::func::DefaultFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Pair;

#[derive(Copy, Clone)]
pub struct FactLib {
    pub make: PrimFuncVal,
    pub get_function: PrimFuncVal,
    pub get_input: PrimFuncVal,
    pub get_output: PrimFuncVal,
}

const FACT: &str = "fact";

pub const MAKE: &str = concatcp!(PREFIX_CELL, FACT, ".make");
pub const GET_FUNCTION: &str = concatcp!(PREFIX_CELL, FACT, ".get_function");
pub const GET_INPUT: &str = concatcp!(PREFIX_CELL, FACT, ".get_input");
pub const GET_OUTPUT: &str = concatcp!(PREFIX_CELL, FACT, ".get_output");

impl Default for FactLib {
    fn default() -> Self {
        Self {
            make: DefaultFunc { fn_: make }.build(),
            get_function: CtxFreeFunc { fn_: get_function }.build(),
            get_input: CtxFreeFunc { fn_: get_input }.build(),
            get_output: CtxFreeFunc { fn_: get_output }.build(),
        }
    }
}

impl CfgMod for FactLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, MAKE, self.make);
        extend_func(cfg, GET_FUNCTION, self.get_function);
        extend_func(cfg, GET_INPUT, self.get_input);
        extend_func(cfg, GET_OUTPUT, self.get_output);
    }
}

fn make(cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{MAKE}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Func(func) = pair.left else {
        return bug!(cfg, "{MAKE}: expected input.left to be a function, but got {}", pair.left);
    };
    let fact = Fact::new(cfg, ctx, func, pair.right);
    if cfg.is_aborted() {
        return Val::default();
    }
    Val::Fact(fact.into())
}

fn get_function(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Fact(fact) = input else {
        return bug!(cfg, "{GET_FUNCTION}: expected input to be a fact, but got {input}");
    };
    Val::Func(fact.func().clone())
}

fn get_input(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Fact(fact) = input else {
        return bug!(cfg, "{GET_INPUT}: expected input to be a fact, but got {input}");
    };
    fact.input().clone()
}

fn get_output(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Fact(fact) = input else {
        return bug!(cfg, "{GET_OUTPUT}: expected input to be a fact, but got {input}");
    };
    fact.output().clone()
}
