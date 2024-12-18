use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    func::mode::ModeFunc,
    rc_wrap,
};

rc_wrap!(pub ModeFuncVal(ModeFunc));

impl Debug for ModeFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("ModeFunc");
        s.field("mode", self.mode());
        s.finish()
    }
}
