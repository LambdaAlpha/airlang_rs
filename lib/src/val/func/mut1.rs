use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    MutFunc,
    func::FuncImpl,
    rc_wrap,
};

rc_wrap!(pub MutFuncVal(MutFunc));

impl Debug for MutFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("MutFunc");
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
