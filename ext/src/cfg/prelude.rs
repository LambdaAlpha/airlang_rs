use airlang::cfg::prelude::CorePrelude;
use airlang::cfg::prelude::Prelude;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;

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
    fn extend(&self, map: &mut Map<Key, Val>) {
        self.core.extend(map);
    }
}
