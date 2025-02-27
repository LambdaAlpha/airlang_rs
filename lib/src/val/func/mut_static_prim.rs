use crate::{
    func::mut_static_prim::MutStaticPrimFunc,
    types::wrap::rc_wrap,
};

rc_wrap!(pub MutStaticPrimFuncVal(MutStaticPrimFunc));

impl_const_func_trait!(MutStaticPrimFuncVal);
