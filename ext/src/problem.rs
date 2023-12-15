use {
    crate::func::ExtFunc,
    airlang::{
        CtxForMutableFn,
        Val,
    },
};

pub trait Solver {
    fn solve(&mut self, ctx: CtxForMutableFn, func: &ExtFunc, output: Val) -> Val;
}

pub(crate) struct DefaultSolver;

impl Solver for DefaultSolver {
    fn solve(&mut self, _ctx: CtxForMutableFn, _func: &ExtFunc, _output: Val) -> Val {
        Val::default()
    }
}
