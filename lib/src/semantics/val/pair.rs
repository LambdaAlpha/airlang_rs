use crate::semantics::val::Val;
use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::Pair;
use crate::type_::wrap::box_wrap;

box_wrap!(pub PairVal(Pair<Val, Val>));

derive_debug!(PairVal(Pair<Val, Val>));

derive_display!(PairVal(Pair<Val, Val>));
