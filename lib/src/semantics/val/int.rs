use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::Int;
use crate::type_::wrap::box_wrap;

box_wrap!(pub IntVal(Int));

derive_debug!(IntVal(Int));

derive_display!(IntVal(Int));
