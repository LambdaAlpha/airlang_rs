use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;

pub fn illegal_input(cfg: &mut Cfg) -> Val {
    abort_bug_with_msg(cfg, "illegal input")
}

pub fn illegal_ctx(cfg: &mut Cfg) -> Val {
    abort_bug_with_msg(cfg, "illegal context")
}

pub fn illegal_cfg(cfg: &mut Cfg) -> Val {
    abort_bug_with_msg(cfg, "illegal config")
}

pub fn abort_bug_with_msg(cfg: &mut Cfg, msg: &str) -> Val {
    crate::semantics::core::abort_bug_with_msg(cfg, msg)
}
