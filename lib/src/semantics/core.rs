pub use self::eval::Eval;
pub use self::form::Form;
pub use self::id::Id;
pub use self::key::PREFIX_CTX;
pub use self::key::PREFIX_ID;
pub use self::key::PREFIX_SHIFT;

_____!();

use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Text;

pub(crate) fn abort_by_bug_with_msg(cfg: &mut Cfg, msg: Text) -> Val {
    abort_by_bug(cfg);
    abort_msg(cfg, msg);
    cfg.abort();
    Val::default()
}

pub(crate) fn abort_by_bug(cfg: &mut Cfg) {
    cfg.export(
        Key::from_str_unchecked(Cfg::ABORT_TYPE),
        Val::Key(Key::from_str_unchecked(Cfg::ABORT_TYPE_BUG)),
    );
}

pub(crate) fn abort_msg(cfg: &mut Cfg, msg: Text) {
    cfg.export(Key::from_str_unchecked(Cfg::ABORT_MSG), Val::Text(msg.into()));
}

mod eval;

mod form;

mod key;

mod ctx;

mod id;
