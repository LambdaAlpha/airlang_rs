use const_format::concatcp;

use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::Val;
use crate::type_::Key;

pub fn illegal_input(cfg: &mut Cfg) -> Val {
    cfg.abort(Key::from_str_unchecked(concatcp!(PREFIX_ID, "illegal_input")));
    Val::default()
}

pub fn illegal_ctx(cfg: &mut Cfg) -> Val {
    cfg.abort(Key::from_str_unchecked(concatcp!(PREFIX_ID, "illegal_context")));
    Val::default()
}

pub fn illegal_cfg(cfg: &mut Cfg) -> Val {
    cfg.abort(Key::from_str_unchecked(concatcp!(PREFIX_ID, "illegal_config")));
    Val::default()
}

pub fn fail(_cfg: &mut Cfg) -> Val {
    Val::default()
}
