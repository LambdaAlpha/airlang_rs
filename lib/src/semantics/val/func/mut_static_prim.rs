use crate::semantics::func::MutStaticPrimFunc;
use crate::type_::wrap::rc_wrap;

rc_wrap!(pub MutStaticPrimFuncVal(MutStaticPrimFunc));

impl_func_trait!(MutStaticPrimFuncVal);
