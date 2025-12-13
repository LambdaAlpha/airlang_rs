use airlang::cfg::prelude::Prelude;
use airlang::cfg::prelude::map_put_func;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::TextVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use airlang_ext::cfg::prelude::StdPrelude;

use crate::cfg::lib::BinLib;

#[derive(Clone)]
pub struct BinPrelude {
    pub std: StdPrelude,
    pub help: TextVal,
    pub call: FreePrimFuncVal,
}

impl BinPrelude {
    pub fn new(lib: &BinLib) -> Self {
        Self {
            std: StdPrelude::new(&lib.std),
            help: lib.repl.help.clone(),
            call: lib.cmd.call.clone(),
        }
    }
}

impl Prelude for BinPrelude {
    fn extend(&self, map: &mut Map<Key, Val>) {
        self.std.extend(map);
        let _ = map.insert(Key::from_str_unchecked("help"), Val::Text(self.help.clone()));
        map_put_func(map, ";", &self.call);
    }
}
