use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    ConstFunc,
    func::FuncImpl,
    rc_wrap,
};

rc_wrap!(pub ConstFuncVal(ConstFunc));

impl Debug for ConstFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("ConstFunc");
        match &self.transformer {
            FuncImpl::Primitive(p) => {
                p.dbg_field(&mut s);
            }
            FuncImpl::Composite(c) => {
                self.dbg_field(&mut s);
                c.dbg_field(&mut s);
                c.dbg_field_ext(&mut s);
            }
        }
        s.finish()
    }
}
