use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Text;

pub fn illegal_input(cfg: &mut Cfg) -> Val {
    abort_bug(cfg, "illegal input")
}

pub fn illegal_ctx(cfg: &mut Cfg) -> Val {
    abort_bug(cfg, "illegal context")
}

pub fn illegal_cfg(cfg: &mut Cfg) -> Val {
    abort_bug(cfg, "illegal config")
}

pub fn abort_bug(cfg: &mut Cfg, message: &str) -> Val {
    cfg.export(
        Key::from_str_unchecked(Cfg::ABORT_TYPE),
        Val::Key(Key::from_str_unchecked(Cfg::ABORT_TYPE_BUG)),
    );
    cfg.export(Key::from_str_unchecked(Cfg::ABORT_MSG), Val::Text(Text::from(message).into()));
    cfg.abort();
    Val::default()
}

pub fn fail(_cfg: &mut Cfg) -> Val {
    Val::default()
}
