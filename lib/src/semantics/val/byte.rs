use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::Byte;
use crate::type_::wrap::box_wrap;

box_wrap!(pub ByteVal(Byte));

derive_debug!(ByteVal(Byte));

derive_display!(ByteVal(Byte));
