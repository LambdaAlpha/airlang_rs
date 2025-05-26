use crate::func::const_static_comp::ConstStaticCompFunc;
use crate::types::wrap::rc_wrap;

rc_wrap!(pub ConstStaticCompFuncVal(ConstStaticCompFunc));

impl_func_trait!(ConstStaticCompFuncVal);
