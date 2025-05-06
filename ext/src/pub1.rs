use airlang::AirExt;
use airlang::Prelude;
use airlang::PreludeCtx;
use airlang::Symbol;
use airlang::TypeMeta;
use airlang::Val;

use crate::prelude::AllPrelude;

#[derive(Default)]
pub struct AirStdExt {
    prelude: AllPrelude,
}

impl Prelude for AirStdExt {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.prelude.put(ctx);
    }
}

impl TypeMeta for AirStdExt {
    fn arbitrary(&self) -> Val {
        Val::default()
    }

    fn arbitrary_type(&self, _type1: Symbol) -> Val {
        Val::default()
    }
}

impl AirExt for AirStdExt {}
