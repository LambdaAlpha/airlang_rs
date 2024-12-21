use crate::{
    func::free_static_prim::FreeStaticPrimFunc,
    types::wrap::rc_wrap,
};

rc_wrap!(pub FreeStaticPrimFuncVal(FreeStaticPrimFunc));

impl_const_func_trait!(FreeStaticPrimFuncVal);
