use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    box_wrap,
    ctx::Ctx,
};

box_wrap!(pub CtxVal(Ctx));

impl Debug for CtxVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ctx::fmt(self, f)
    }
}
