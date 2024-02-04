use crate::{
    ctx::NameMap,
    extension::UnitExt,
    prelude::{
        Named,
        Prelude,
    },
    ValExt,
};

#[derive(Clone)]
pub(crate) struct ExtPrelude {
    pub(crate) unit: Named<Box<dyn ValExt>>,
}

#[allow(clippy::derivable_impls)]
impl Default for ExtPrelude {
    fn default() -> Self {
        ExtPrelude { unit: unit() }
    }
}

impl Prelude for ExtPrelude {
    fn put(&self, m: &mut NameMap) {
        self.unit.put(m);
    }
}

fn unit() -> Named<Box<dyn ValExt>> {
    Named::new("extension.unit", Box::new(UnitExt))
}
