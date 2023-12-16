use {
    crate::{
        prelude::io::IoPrelude,
        ExtFunc,
    },
    airlang::Symbol,
    std::collections::HashMap,
};

#[derive(Default)]
pub(crate) struct AllPrelude {
    pub(crate) io: IoPrelude,
}

impl Prelude for AllPrelude {
    fn put(self, m: &mut HashMap<Symbol, ExtFunc>) {
        self.io.put(m);
    }
}

pub(crate) trait Prelude {
    fn put(self, m: &mut HashMap<Symbol, ExtFunc>);
}

pub(crate) struct NamedExtFunc {
    pub(crate) name: &'static str,
    pub(crate) func: ExtFunc,
}

impl NamedExtFunc {
    pub(crate) fn new(name: &'static str, func: ExtFunc) -> Self {
        Self { name, func }
    }

    pub(crate) fn put(self, m: &mut HashMap<Symbol, ExtFunc>) {
        let name = unsafe { Symbol::from_str_unchecked(self.name) };
        m.insert(name, self.func);
    }
}

pub(crate) mod io;
