use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    CellFunc,
    box_wrap,
    func::FuncImpl,
};

box_wrap!(pub CellFuncVal(CellFunc));

impl Debug for CellFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("CellFunc");
        match &self.transformer {
            FuncImpl::Primitive(p) => {
                p.dbg_field(&mut s);
                p.dbg_field_ext(&mut s);
            }
            FuncImpl::Composite(c) => {
                self.dbg_field(&mut s);
                c.dbg_field(&mut s);
            }
        }
        s.finish()
    }
}
