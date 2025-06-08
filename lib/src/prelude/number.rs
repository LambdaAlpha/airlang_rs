use super::Prelude;
use super::PreludeCtx;

// todo design
#[derive(Clone)]
pub struct NumberPrelude {}

#[expect(clippy::derivable_impls)]
impl Default for NumberPrelude {
    fn default() -> Self {
        NumberPrelude {}
    }
}

impl Prelude for NumberPrelude {
    fn put(&self, _ctx: &mut dyn PreludeCtx) {}
}
