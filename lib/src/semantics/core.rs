pub(crate) use self::eval::Eval;
pub(crate) use self::form::Form;
pub(crate) use self::id::Id;
#[expect(unused_imports)]
pub(crate) use self::key::PREFIX_CTX;
pub(crate) use self::key::PREFIX_ID;
#[expect(unused_imports)]
pub(crate) use self::key::PREFIX_SHIFT;

_____!();

use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Text;

pub(crate) fn abort_bug_with_msg(cfg: &mut Cfg, msg: &str) -> Val {
    abort_bug(cfg);
    abort_msg(cfg, msg);
    cfg.abort();
    Val::default()
}

pub(crate) fn abort_bug(cfg: &mut Cfg) {
    cfg.export(
        Key::from_str_unchecked(Cfg::ABORT_TYPE),
        Val::Key(Key::from_str_unchecked(Cfg::ABORT_TYPE_BUG)),
    );
}

pub(crate) fn abort_msg(cfg: &mut Cfg, msg: &str) {
    cfg.export(Key::from_str_unchecked(Cfg::ABORT_MSG), Val::Text(Text::from(msg).into()));
}

mod eval;

mod form;

mod key;

mod ctx;

mod id;
