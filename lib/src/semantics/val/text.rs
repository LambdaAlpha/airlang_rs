use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::Text;
use crate::type_::wrap::box_wrap;

box_wrap!(pub TextVal(Text));

derive_debug!(TextVal(Text));

derive_display!(TextVal(Text));
