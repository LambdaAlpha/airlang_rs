use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::DynFunc;
use crate::semantics::val::FuncVal;
use crate::semantics::val::Val;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Fact {
    func: FuncVal,
    input: Val,
    output: Val,
}

impl Fact {
    pub fn new(cfg: &mut Cfg, ctx: Ctx<Val>, func: FuncVal, input: Val) -> Self {
        let output = func.call(cfg, ctx, input.clone());
        Self { func, input, output }
    }

    pub fn func(&self) -> &FuncVal {
        &self.func
    }

    pub fn input(&self) -> &Val {
        &self.input
    }

    pub fn output(&self) -> &Val {
        &self.output
    }
}
