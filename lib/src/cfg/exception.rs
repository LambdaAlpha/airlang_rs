use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;

pub fn illegal_input(_cfg: &mut Cfg) -> Val {
    Val::default()
}

pub fn illegal_ctx(_cfg: &mut Cfg) -> Val {
    Val::default()
}

pub fn illegal_cfg(_cfg: &mut Cfg) -> Val {
    Val::default()
}

pub fn fail(_cfg: &mut Cfg) -> Val {
    Val::default()
}
