use crate::{
    Map,
    Symbol,
    ValExt,
    ctx::map::CtxValue,
    extension::UnitExt,
    prelude::{
        Named,
        Prelude,
    },
};

#[derive(Clone)]
pub(crate) struct ExtPrelude {
    pub(crate) unit: Named<Box<dyn ValExt>>,
}

impl Default for ExtPrelude {
    fn default() -> Self {
        ExtPrelude { unit: unit() }
    }
}

impl Prelude for ExtPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.unit.put(m);
    }
}

fn unit() -> Named<Box<dyn ValExt>> {
    let id = "extension.unit";
    let v = Box::new(UnitExt);
    Named::new(id, v)
}
