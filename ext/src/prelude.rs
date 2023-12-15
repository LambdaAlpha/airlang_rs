use {
    crate::func::ExtFunc,
    airlang::Symbol,
    std::collections::HashMap,
};

thread_local!(pub(crate) static PRELUDE: AllPrelude = AllPrelude::default());

#[derive(Default)]
pub(crate) struct AllPrelude {}

impl Prelude for AllPrelude {
    fn put(&self, _m: &mut HashMap<Symbol, ExtFunc>) {}
}

pub(crate) trait Prelude {
    fn put(&self, m: &mut HashMap<Symbol, ExtFunc>);
}

#[derive(Clone)]
pub(crate) struct NamedExtFunc {
    pub(crate) name: &'static str,
    pub(crate) func: ExtFunc,
}

#[allow(unused)]
impl NamedExtFunc {
    pub(crate) fn new(name: &'static str, func: ExtFunc) -> Self {
        Self { name, func }
    }

    pub(crate) fn put(&self, m: &mut HashMap<Symbol, ExtFunc>) {
        let name = unsafe { Symbol::from_str_unchecked(self.name) };
        m.insert(name, self.func.clone());
    }
}
