use crate::semantics::val::Val;
use crate::type_::Pair;
use crate::type_::wrap::box_wrap;

box_wrap!(pub PairVal(Pair<Val, Val>));
