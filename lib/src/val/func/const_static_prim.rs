use crate::{
    func::const_static_prim::ConstStaticPrimFunc,
    types::wrap::rc_wrap,
};

rc_wrap!(pub ConstStaticPrimFuncVal(ConstStaticPrimFunc));

impl_const_func_trait!(ConstStaticPrimFuncVal);
