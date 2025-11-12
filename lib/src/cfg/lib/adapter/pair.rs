use crate::cfg::lib::adapter::CoreAdapter;
use crate::semantics::core::PairForm;
use crate::semantics::val::Val;
use crate::type_::Map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PairAdapter {
    pub some: Map<Val, CoreAdapter>,
    pub first: CoreAdapter,
    pub second: CoreAdapter,
}

impl PairAdapter {
    pub(crate) fn form(&self) -> PairForm<'_, CoreAdapter, CoreAdapter, CoreAdapter> {
        PairForm { some: &self.some, first: &self.first, second: &self.second }
    }
}
