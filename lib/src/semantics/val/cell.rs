use crate::semantics::val::Val;
use crate::type_::Cell;
use crate::type_::wrap::box_wrap;

box_wrap!(pub CellVal(Cell<Val>));
