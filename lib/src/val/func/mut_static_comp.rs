use crate::{
    func::mut_static_comp::MutStaticCompFunc,
    types::wrap::rc_wrap,
};

rc_wrap!(pub MutStaticCompFuncVal(MutStaticCompFunc));

impl_const_func_trait!(MutStaticCompFuncVal);
