use crate::{
    Bit,
    ConstFnCtx,
    FuncMode,
    FuncVal,
    Pair,
    Val,
    ctx::main::MainCtx,
    prelude::{
        Named,
        Prelude,
        PreludeCtx,
        named_const_fn,
        ref_pair_mode,
    },
};

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
    let f = fn_is_ext;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_is_ext(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
        let is_ext = matches!(val, Val::Ext(_));
        Val::Bit(Bit::new(is_ext))
    })
}
