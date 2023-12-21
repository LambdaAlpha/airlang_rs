use {
    crate::{
        init_ctx,
        prelude::{
            put_func,
            Prelude,
        },
    },
    airlang::{
        initial_ctx,
        CtxForMutableFn,
        EvalMode,
        IoMode,
        MutableCtx,
        Symbol,
        Val,
    },
    airlang_ext::{
        ExtFn,
        ExtFunc,
    },
    std::rc::Rc,
};

pub(crate) struct EvalPrelude {
    pub(crate) reset: Rc<ExtFunc>,
}

impl Default for EvalPrelude {
    fn default() -> Self {
        Self { reset: reset() }
    }
}

impl Prelude for EvalPrelude {
    fn put(&self, mut ctx: MutableCtx) {
        put_func(&self.reset, ctx.reborrow());
    }
}

fn reset() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("repl.reset") };
    let ext_fn = ExtFn::new_mutable(fn_reset);
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::Value);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
}

fn fn_reset(ctx: CtxForMutableFn, _input: Val) -> Val {
    let CtxForMutableFn::Mutable(mut ctx) = ctx else {
        eprintln!("Unable to reset context, context is immutable.");
        return Val::default();
    };
    let mut initial_ctx = initial_ctx();
    init_ctx(MutableCtx::new(&mut initial_ctx));
    ctx.set(initial_ctx);
    Val::default()
}
