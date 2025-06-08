use crate::semantics::func::FreeStaticPrimFunc;
use crate::type_::wrap::rc_wrap;

rc_wrap!(pub FreeStaticPrimFuncVal(FreeStaticPrimFunc));

impl_func_trait!(FreeStaticPrimFuncVal);
