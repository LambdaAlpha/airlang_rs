use crate::semantics::val::Val;
use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::Quote;
use crate::type_::wrap::box_wrap;

box_wrap!(pub QuoteVal(Quote<Val>));

derive_debug!(QuoteVal(Quote<Val>));

derive_display!(QuoteVal(Quote<Val>));
