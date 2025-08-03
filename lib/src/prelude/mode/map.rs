use super::Mode;
use crate::semantics::core::MapForm;
use crate::semantics::val::Val;
use crate::type_::Map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapMode {
    pub some: Map<Val, Mode>,
    pub else_: Mode,
}

impl MapMode {
    pub(crate) fn form(&self) -> MapForm<'_, Val, Mode, Mode> {
        MapForm { some: &self.some, else_: &self.else_ }
    }
}
