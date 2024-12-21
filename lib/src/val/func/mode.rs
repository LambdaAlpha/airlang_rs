use crate::{
    func::mode::ModeFunc,
    rc_wrap,
};

rc_wrap!(pub ModeFuncVal(ModeFunc));

impl_const_func_trait!(ModeFuncVal);
