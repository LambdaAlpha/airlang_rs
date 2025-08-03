use super::Mode;
use crate::semantics::core::ListForm;
use crate::type_::List;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListMode {
    pub head: List<Mode>,
    pub tail: Mode,
}

impl ListMode {
    pub(crate) fn form(&self) -> ListForm<'_, Mode, Mode> {
        ListForm { head: &self.head, tail: &self.tail }
    }
}
