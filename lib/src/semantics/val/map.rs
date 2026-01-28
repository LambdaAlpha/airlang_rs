use crate::semantics::val::Val;
use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::wrap::box_wrap;

box_wrap!(pub MapVal(Map<Key, Val>));

derive_debug!(MapVal(Map<Key, Val>));

derive_display!(MapVal(Map<Key, Val>));
