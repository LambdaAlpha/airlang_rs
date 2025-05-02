use crate::{
    ValExt,
    extension::UnitExt,
    prelude::{
        Named,
        Prelude,
        PreludeCtx,
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
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.unit.put(ctx);
    }
}

fn unit() -> Named<Box<dyn ValExt>> {
    let id = "extension.unit";
    let v = Box::new(UnitExt);
    Named::new(id, v)
}
