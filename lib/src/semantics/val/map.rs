use crate::semantics::val::Val;
use crate::type_::Map;
use crate::type_::wrap::box_wrap;

box_wrap!(pub MapVal(Map<Val, Val>));
