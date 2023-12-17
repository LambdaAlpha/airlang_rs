use {
    crate::prelude::{
        NamedExtFunc,
        Prelude,
        PreludeMap,
    },
    airlang::{
        EvalMode,
        IoMode,
        Val,
    },
    airlang_ext::{
        ExtFn,
        ExtFunc,
    },
};

pub(crate) struct ReplPrelude {
    pub(crate) exit: NamedExtFunc,
}

impl Default for ReplPrelude {
    fn default() -> Self {
        Self { exit: exit() }
    }
}

impl Prelude for ReplPrelude {
    fn put(self, m: &mut impl PreludeMap) {
        self.exit.put(m);
    }
}

fn exit() -> NamedExtFunc {
    let ext_fn = ExtFn::new_free(fn_exit);
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::Value);
    let func = ExtFunc::new(input_mode, output_mode, ext_fn);
    NamedExtFunc::new("repl.exit", func)
}

fn fn_exit(_input: Val) -> Val {
    std::process::exit(0)
}
