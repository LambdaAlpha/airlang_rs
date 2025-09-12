use crate::cfg::adapter::core::CoreAdapter;
use crate::semantics::core::MapForm;
use crate::semantics::val::Val;
use crate::type_::Map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapAdapter {
    pub some: Map<Val, CoreAdapter>,
    pub else_: CoreAdapter,
}

impl MapAdapter {
    pub(crate) fn form(&self) -> MapForm<'_, Val, CoreAdapter, CoreAdapter> {
        MapForm { some: &self.some, else_: &self.else_ }
    }
}
