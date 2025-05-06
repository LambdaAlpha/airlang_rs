use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;

#[derive(Clone)]
pub(crate) struct NumberPrelude {}

#[expect(clippy::derivable_impls)]
impl Default for NumberPrelude {
    fn default() -> Self {
        NumberPrelude {}
    }
}

impl Prelude for NumberPrelude {
    fn put(&self, _ctx: &mut dyn PreludeCtx) {}
}
