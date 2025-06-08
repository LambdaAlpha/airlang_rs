use crate::semantics::func::ModeFunc;
use crate::type_::wrap::rc_wrap;

rc_wrap!(pub ModeFuncVal(ModeFunc));

impl_func_trait!(ModeFuncVal);
