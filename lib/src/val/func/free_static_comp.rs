use crate::{
    func::free_static_comp::FreeStaticCompFunc,
    rc_wrap,
};

rc_wrap!(pub FreeStaticCompFuncVal(FreeStaticCompFunc));

impl_const_func_trait!(FreeStaticCompFuncVal);
