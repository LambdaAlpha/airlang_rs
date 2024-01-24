use crate::{
    ctx::NameMap,
    prelude::{
        Named,
        Prelude,
    },
    Symbol,
};

#[derive(Clone)]
pub(crate) struct SymbolPrelude {
    pub(crate) empty: Named<Symbol>,
}

#[allow(clippy::derivable_impls)]
impl Default for SymbolPrelude {
    fn default() -> Self {
        SymbolPrelude { empty: empty() }
    }
}

impl Prelude for SymbolPrelude {
    fn put(&self, m: &mut NameMap) {
        self.empty.put(m);
    }
}

fn empty() -> Named<Symbol> {
    Named::new("", Symbol::from_str(""))
}
