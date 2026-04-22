use const_format::concatcp;

use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Text;

pub const ABORT_TYPE: &str = concatcp!(PREFIX_CELL, "error.abort.type");
pub const ABORT_MSG: &str = concatcp!(PREFIX_CELL, "error.abort.message");

pub const ABORT_TYPE_BUG: &str = concatcp!(PREFIX_CELL, "bug");

pub fn abort_by_bug_with_msg(cfg: &mut Cfg, msg: Text) -> Val {
    abort_by_bug(cfg);
    set_abort_msg(cfg, msg);
    cfg.abort();
    Val::default()
}

pub fn abort_by_bug(cfg: &mut Cfg) {
    cfg.export(
        Key::from_str_unchecked(ABORT_TYPE),
        Val::Key(Key::from_str_unchecked(ABORT_TYPE_BUG)),
    );
}

pub fn set_abort_msg(cfg: &mut Cfg, msg: Text) {
    cfg.export(Key::from_str_unchecked(ABORT_MSG), Val::Text(msg.into()));
}

#[macro_export]
macro_rules! bug {
     ($cfg: tt, $($arg:tt)*) => {
        $crate::cfg::error::abort_by_bug_with_msg($cfg, format!($($arg)*).into())
     };
}
