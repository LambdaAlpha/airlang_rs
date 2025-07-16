use crate::semantics::val::Val;
use crate::type_::Task;
use crate::type_::wrap::box_wrap;

box_wrap!(pub TaskVal(Task<Val, Val, Val>));
