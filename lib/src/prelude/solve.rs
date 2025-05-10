use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;

#[derive(Clone)]
pub(crate) struct SolvePrelude {}

#[expect(clippy::derivable_impls)]
impl Default for SolvePrelude {
    fn default() -> Self {
        SolvePrelude {}
    }
}

impl Prelude for SolvePrelude {
    fn put(&self, _ctx: &mut dyn PreludeCtx) {}
}
