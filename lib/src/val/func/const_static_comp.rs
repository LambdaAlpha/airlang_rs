use crate::{
    func::const_static_comp::ConstStaticCompFunc,
    types::wrap::rc_wrap,
};

rc_wrap!(pub ConstStaticCompFuncVal(ConstStaticCompFunc));

impl_const_func_trait!(ConstStaticCompFuncVal);
