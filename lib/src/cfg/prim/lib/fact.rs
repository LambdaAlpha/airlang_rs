use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::CtxFreeInputAwareFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Cell;
use crate::type_::Pair;

#[derive(Copy, Clone)]
pub struct FactLib {
    pub put: PrimFuncVal,
    pub call: PrimFuncVal,
    pub solve: PrimFuncVal,
    pub exist: PrimFuncVal,
}

const FACT: &str = "fact";

pub const PUT: &str = concatcp!(PREFIX_CELL, FACT, ".put");
pub const CALL: &str = concatcp!(PREFIX_CELL, FACT, ".call");
pub const SOLVE: &str = concatcp!(PREFIX_CELL, FACT, ".solve");
pub const EXIST: &str = concatcp!(PREFIX_CELL, FACT, ".exist");

impl Default for FactLib {
    fn default() -> Self {
        Self {
            put: CtxFreeInputAwareFunc { fn_: put }.build(),
            call: CtxFreeInputAwareFunc { fn_: call }.build(),
            solve: CtxFreeInputAwareFunc { fn_: solve }.build(),
            exist: CtxFreeInputAwareFunc { fn_: exist }.build(),
        }
    }
}

impl CfgMod for FactLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, PUT, self.put);
        extend_func(cfg, CALL, self.call);
        extend_func(cfg, SOLVE, self.solve);
        extend_func(cfg, EXIST, self.exist);
    }
}

pub fn put(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{PUT}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Func(func) = pair.left else {
        return bug!(cfg, "{PUT}: expected input.left to be a function, but got {}", pair.left);
    };
    cfg.fact_put(&mut Val::default(), func, pair.right);
    Val::default()
}

pub fn call(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{CALL}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Func(func) = pair.left else {
        return bug!(cfg, "{CALL}: expected input.left to be a function, but got {}", pair.left);
    };
    match cfg.fact_call(func, pair.right) {
        Some(output) => Val::Cell(Cell::new(output).into()),
        None => Val::default(),
    }
}

pub fn solve(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{SOLVE}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Func(func) = pair.left else {
        return bug!(cfg, "{SOLVE}: expected input.left to be a function, but got {}", pair.left);
    };
    match cfg.fact_solve(func, pair.right) {
        Some(input) => Val::Cell(Cell::new(input).into()),
        None => Val::default(),
    }
}

pub fn exist(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{EXIST}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Func(func) = pair.left else {
        return bug!(cfg, "{EXIST}: expected input.left to be a function, but got {}", pair.left);
    };
    let Val::Pair(pair) = pair.right else {
        return bug!(cfg, "{EXIST}: expected input.right to be a pair, but got {}", pair.right);
    };
    let pair = Pair::from(pair);
    let exist = cfg.fact_exist(func, pair.left, pair.right);
    Val::Bit(Bit::from(exist))
}
