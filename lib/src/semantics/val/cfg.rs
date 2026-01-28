use crate::semantics::cfg::Cfg;
use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::wrap::box_wrap;

box_wrap!(pub CfgVal(Cfg));

derive_debug!(CfgVal(Cfg));

derive_display!(CfgVal(Cfg));
