use crate::semantics::val::Val;
use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::Cell;
use crate::type_::wrap::box_wrap;

box_wrap!(pub CellVal(Cell<Val>));

derive_debug!(CellVal(Cell<Val>));

derive_display!(CellVal(Cell<Val>));
