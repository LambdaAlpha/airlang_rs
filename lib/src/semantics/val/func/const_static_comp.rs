use crate::semantics::func::ConstStaticCompFunc;
use crate::type_::wrap::rc_wrap;

rc_wrap!(pub ConstStaticCompFuncVal(ConstStaticCompFunc));

impl_func_trait!(ConstStaticCompFuncVal);
