use {
    crate::prelude::{
        eval::EvalPrelude,
        repl::ReplPrelude,
    },
    airlang::Symbol,
    airlang_ext::ExtFunc,
};

#[derive(Default)]
pub(crate) struct AllPrelude {
    pub(crate) repl: ReplPrelude,
    pub(crate) eval: EvalPrelude,
}

pub(crate) trait PreludeMap {
    fn put(&mut self, name: Symbol, func: ExtFunc);
}

pub(crate) trait Prelude {
    fn put(self, map: &mut impl PreludeMap);
}

impl Prelude for AllPrelude {
    fn put(self, m: &mut impl PreludeMap) {
        self.repl.put(m);
        self.eval.put(m);
    }
}

pub(crate) struct NamedExtFunc {
    pub(crate) name: &'static str,
    pub(crate) func: ExtFunc,
}

impl NamedExtFunc {
    pub(crate) fn new(name: &'static str, func: ExtFunc) -> Self {
        Self { name, func }
    }

    pub(crate) fn put(self, map: &mut impl PreludeMap) {
        let name = unsafe { Symbol::from_str_unchecked(self.name) };
        map.put(name, self.func);
    }
}

mod repl;

mod eval;
