use airlang::cfg::prim::prelude::Prelude;
use airlang::cfg::prim::prelude::map_put_func;
use airlang::semantics::val::PrimFuncVal;
use airlang::semantics::val::TextVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use airlang_ext::cfg::prim::prelude::ExtPrimPrelude;

use crate::cfg::prim::lib::BinPrimLib;

#[derive(Clone)]
pub struct BinPrimPrelude {
    pub ext: ExtPrimPrelude,
    pub help: TextVal,
    pub call: PrimFuncVal,
}

impl BinPrimPrelude {
    pub fn new(lib: &BinPrimLib) -> Self {
        Self {
            ext: ExtPrimPrelude::new(&lib.ext),
            help: lib.repl.help.clone(),
            call: lib.cmd.call.clone(),
        }
    }
}

impl Prelude for BinPrimPrelude {
    fn extend(&self, map: &mut Map<Key, Val>) {
        self.ext.extend(map);
        let _ = map.insert(Key::from_str_unchecked("help"), Val::Text(self.help.clone()));
        map_put_func(map, ";", &self.call);
    }
}
