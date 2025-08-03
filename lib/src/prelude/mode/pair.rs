use super::Mode;
use crate::semantics::core::PairForm;
use crate::semantics::val::Val;
use crate::type_::Map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PairMode {
    pub some: Map<Val, Mode>,
    pub first: Mode,
    pub second: Mode,
}

impl PairMode {
    pub(crate) fn form(&self) -> PairForm<'_, Mode, Mode, Mode> {
        PairForm { some: &self.some, first: &self.first, second: &self.second }
    }
}
