use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use crate::semantics::fact::Fact;
use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::wrap::rc_wrap;

rc_wrap!(pub FactVal(Fact));

derive_debug!(FactVal(Fact));

derive_display!(FactVal(Fact));

impl PartialEq for FactVal {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for FactVal {}

impl Hash for FactVal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}
