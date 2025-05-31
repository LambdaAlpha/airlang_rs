use crate::Bit;
use crate::ConstRef;
use crate::FuncMode;
use crate::FuncVal;
use crate::Val;
use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::named_const_fn;

#[derive(Clone)]
pub(crate) struct ExtPrelude {
    pub(crate) is_ext: Named<FuncVal>,
}

impl Default for ExtPrelude {
    fn default() -> Self {
        ExtPrelude { is_ext: is_ext() }
    }
}

impl Prelude for ExtPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.is_ext.put(ctx);
    }
}

fn is_ext() -> Named<FuncVal> {
    let id = "is_extension";
    let f = const_impl(fn_is_ext);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_is_ext(ctx: ConstRef<Val>, _input: Val) -> Val {
    let is_ext = matches!(&*ctx, Val::Ext(_));
    Val::Bit(Bit::new(is_ext))
}
