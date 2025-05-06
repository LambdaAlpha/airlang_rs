use crate::FuncVal;
use crate::MutFnCtx;
use crate::Val;

pub(crate) fn inverse(_ctx: MutFnCtx, _func: FuncVal, _output: Val) -> Val {
    Val::default()
}

pub(crate) fn imply(_ctx: MutFnCtx, _first: Val, _second: Val) -> Val {
    Val::default()
}
