use crate::semantics::val::Val;
use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::Solve;
use crate::type_::wrap::box_wrap;

box_wrap!(pub SolveVal(Solve<Val, Val>));

derive_debug!(SolveVal(Solve<Val, Val>));

derive_display!(SolveVal(Solve<Val, Val>));
