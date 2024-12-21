use crate::{
    func::free_static_prim::FreeStaticPrimFunc,
    rc_wrap,
};

rc_wrap!(pub FreeStaticPrimFuncVal(FreeStaticPrimFunc));

impl_const_func_trait!(FreeStaticPrimFuncVal);
