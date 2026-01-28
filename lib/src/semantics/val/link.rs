use std::cell::BorrowError;
use std::cell::BorrowMutError;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use derive_more::From;

use crate::semantics::val::Val;

#[derive(Clone, PartialEq, Eq, From)]
pub struct LinkVal {
    cell: Rc<RefCell<Val>>,
    const_: bool,
}

impl LinkVal {
    pub fn new(val: Val, const_: bool) -> Self {
        Self { cell: Rc::new(RefCell::new(val)), const_ }
    }

    pub fn is_const(&self) -> bool {
        self.const_
    }

    pub(crate) fn ptr_addr(&self) -> usize {
        Rc::as_ptr(&self.cell).addr()
    }

    pub fn try_borrow(&self) -> Result<Ref<'_, Val>, BorrowError> {
        self.cell.try_borrow()
    }

    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, Val>, BorrowMutError> {
        self.cell.try_borrow_mut()
    }
}
