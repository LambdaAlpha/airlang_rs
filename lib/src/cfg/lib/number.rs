use super::Library;
use crate::semantics::ctx::Ctx;

// todo design
#[derive(Clone)]
pub struct NumberLib {}

#[expect(clippy::derivable_impls)]
impl Default for NumberLib {
    fn default() -> Self {
        NumberLib {}
    }
}

impl Library for NumberLib {
    fn prelude(&self, _ctx: &mut Ctx) {}
}
