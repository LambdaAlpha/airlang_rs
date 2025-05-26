use crate::func::free_static_comp::FreeStaticCompFunc;
use crate::types::wrap::rc_wrap;

rc_wrap!(pub FreeStaticCompFuncVal(FreeStaticCompFunc));

impl_func_trait!(FreeStaticCompFuncVal);
