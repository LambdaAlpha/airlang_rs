use crate::cfg::adapter::core::CoreAdapter;
use crate::semantics::core::ListForm;
use crate::type_::List;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListAdapter {
    pub head: List<CoreAdapter>,
    pub tail: CoreAdapter,
}

impl ListAdapter {
    pub(crate) fn form(&self) -> ListForm<'_, CoreAdapter, CoreAdapter> {
        ListForm { head: &self.head, tail: &self.tail }
    }
}
