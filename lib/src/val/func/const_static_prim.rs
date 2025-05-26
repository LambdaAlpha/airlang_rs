use crate::func::const_static_prim::ConstStaticPrimFunc;
use crate::types::wrap::rc_wrap;

rc_wrap!(pub ConstStaticPrimFuncVal(ConstStaticPrimFunc));

impl_func_trait!(ConstStaticPrimFuncVal);
