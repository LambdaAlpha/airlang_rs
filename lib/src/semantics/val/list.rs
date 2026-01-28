use crate::semantics::val::Val;
use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::List;
use crate::type_::wrap::box_wrap;

box_wrap!(pub ListVal(List<Val>));

derive_debug!(ListVal(List<Val>));

derive_display!(ListVal(List<Val>));
