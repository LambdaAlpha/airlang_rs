use airlang::{
    AirExt,
    Prelude,
    PreludeCtx,
    Symbol,
    TypeMeta,
    Val,
};
use airlang_ext::AirStdExt;

use crate::prelude::AllPrelude;

#[derive(Default)]
pub(crate) struct BinExt {
    ext: AirStdExt,
    prelude: AllPrelude,
}

impl Prelude for BinExt {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.ext.put(ctx);
        self.prelude.put(ctx);
    }
}

impl TypeMeta for BinExt {
    fn arbitrary(&self) -> Val {
        self.ext.arbitrary()
    }

    fn arbitrary_type(&self, type1: Symbol) -> Val {
        self.ext.arbitrary_type(type1)
    }
}

impl AirExt for BinExt {}
