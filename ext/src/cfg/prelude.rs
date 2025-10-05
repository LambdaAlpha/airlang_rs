use airlang::cfg::prelude::CorePrelude;
use airlang::cfg::prelude::Prelude;
use airlang::semantics::memo::Memo;

use crate::cfg::lib::StdLib;

#[derive(Clone)]
pub struct StdPrelude {
    core: CorePrelude,
}

impl StdPrelude {
    pub fn new(lib: &StdLib) -> Self {
        Self { core: CorePrelude::new(&lib.core) }
    }
}

impl Prelude for StdPrelude {
    fn extend(&self, memo: &mut Memo) {
        self.core.extend(memo);
    }
}
