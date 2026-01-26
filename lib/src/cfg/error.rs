use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;

pub fn abort_by_bug_with_msg(cfg: &mut Cfg, msg: String) -> Val {
    crate::semantics::core::abort_by_bug_with_msg(cfg, msg.into())
}

#[macro_export]
macro_rules! bug {
     ($cfg: tt, $($arg:tt)*) => {
        $crate::cfg::error::abort_by_bug_with_msg($cfg, format!($($arg)*).into())
     };
}
