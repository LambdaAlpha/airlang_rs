use crate::func::mode::ModeFunc;
use crate::types::wrap::rc_wrap;

rc_wrap!(pub ModeFuncVal(ModeFunc));

impl_func_trait!(ModeFuncVal);
