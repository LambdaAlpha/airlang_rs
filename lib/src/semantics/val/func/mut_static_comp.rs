use crate::semantics::func::MutStaticCompFunc;
use crate::type_::wrap::rc_wrap;

rc_wrap!(pub MutStaticCompFuncVal(MutStaticCompFunc));

impl_func_trait!(MutStaticCompFuncVal);
