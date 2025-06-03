use crate::Bit;
use crate::ConstRef;
use crate::ConstStaticPrimFuncVal;
use crate::FuncMode;
use crate::Val;
use crate::prelude::DynFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;

#[derive(Clone)]
pub(crate) struct ExtPrelude {
    pub(crate) is_ext: ConstStaticPrimFuncVal,
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

fn is_ext() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "is_extension",
        f: const_impl(fn_is_ext),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_is_ext(ctx: ConstRef<Val>, _input: Val) -> Val {
    let is_ext = matches!(&*ctx, Val::Ext(_));
    Val::Bit(Bit::new(is_ext))
}
