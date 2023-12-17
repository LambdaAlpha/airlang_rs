use {
    crate::prelude::{
        NamedExtFunc,
        Prelude,
        PreludeMap,
    },
    airlang::{
        initial_ctx,
        CtxForMutableFn,
        EvalMode,
        IoMode,
        Val,
    },
    airlang_ext::{
        ExtFn,
        ExtFunc,
    },
};

pub(crate) struct EvalPrelude {
    pub(crate) reset: NamedExtFunc,
}

impl Default for EvalPrelude {
    fn default() -> Self {
        Self { reset: reset() }
    }
}

impl Prelude for EvalPrelude {
    fn put(self, m: &mut impl PreludeMap) {
        self.reset.put(m);
    }
}

fn reset() -> NamedExtFunc {
    let ext_fn = ExtFn::new_mutable(fn_reset);
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::Value);
    let func = ExtFunc::new(input_mode, output_mode, ext_fn);
    NamedExtFunc::new("repl.reset", func)
}

fn fn_reset(ctx: CtxForMutableFn, _input: Val) -> Val {
    let CtxForMutableFn::Mutable(mut ctx) = ctx else {
        eprintln!("Unable to reset context, context is immutable.");
        return Val::default();
    };
    let initial_ctx = initial_ctx();
    ctx.set(initial_ctx);
    Val::default()
}
