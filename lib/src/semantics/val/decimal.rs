use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::Decimal;
use crate::type_::wrap::box_wrap;

box_wrap!(pub DecimalVal(Decimal));

derive_debug!(DecimalVal(Decimal));

derive_display!(DecimalVal(Decimal));
