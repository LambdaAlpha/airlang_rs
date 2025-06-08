use crate::semantics::val::Val;
use crate::type_::List;
use crate::type_::wrap::box_wrap;

box_wrap!(pub ListVal(List<Val>));
