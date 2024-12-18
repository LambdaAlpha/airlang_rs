use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    FreeFunc,
    func::FuncImpl,
    rc_wrap,
};

rc_wrap!(pub FreeFuncVal(FreeFunc));

impl Debug for FreeFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("FreeFunc");
        match &self.transformer {
            FuncImpl::Primitive(p) => {
                p.dbg_field(&mut s);
            }
            FuncImpl::Composite(c) => {
                self.dbg_field(&mut s);
                c.dbg_field(&mut s);
            }
        }
        s.finish()
    }
}
