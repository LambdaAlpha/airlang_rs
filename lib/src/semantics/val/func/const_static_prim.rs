use crate::semantics::func::ConstStaticPrimFunc;
use crate::type_::wrap::rc_wrap;

rc_wrap!(pub ConstStaticPrimFuncVal(ConstStaticPrimFunc));

impl_func_trait!(ConstStaticPrimFuncVal);
