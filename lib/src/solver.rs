use std::cell::RefCell;

use crate::FuncVal;
use crate::Val;
use crate::ctx::ref1::CtxMeta;

thread_local!(pub(crate) static SOLVER: RefCell<Option<FuncVal>> = RefCell::default());

pub(crate) fn reverse<'a, Ctx>(_ctx: Ctx, _func: FuncVal, _output: Val) -> Val
where Ctx: CtxMeta<'a> {
    Val::default()
}
