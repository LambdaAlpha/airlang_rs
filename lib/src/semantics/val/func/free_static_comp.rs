use crate::semantics::func::FreeStaticCompFunc;
use crate::type_::wrap::rc_wrap;

rc_wrap!(pub FreeStaticCompFuncVal(FreeStaticCompFunc));

impl_func_trait!(FreeStaticCompFuncVal);
