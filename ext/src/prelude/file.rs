use {
    crate::{
        prelude::{
            NamedExtFunc,
            Prelude,
        },
        ExtFn,
        ExtFunc,
    },
    airlang::{
        CtxForConstFn,
        EvalMode,
        IoMode,
        Str,
        Symbol,
        Val,
    },
    std::collections::HashMap,
};

pub(crate) struct FilePrelude {
    pub(crate) read_to_string: NamedExtFunc,
}

impl Default for FilePrelude {
    fn default() -> Self {
        Self {
            read_to_string: read_to_string(),
        }
    }
}

impl Prelude for FilePrelude {
    fn put(self, m: &mut HashMap<Symbol, ExtFunc>) {
        self.read_to_string.put(m);
    }
}

fn read_to_string() -> NamedExtFunc {
    let ext_fn = ExtFn::new_const(fn_read_to_string);
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    let func = ExtFunc::new(input_mode, output_mode, ext_fn);
    NamedExtFunc::new("file.read_to_string", func)
}

fn fn_read_to_string(ctx: CtxForConstFn, input: Val) -> Val {
    let result = match input {
        Val::String(path) => std::fs::read_to_string(&*path),
        Val::Symbol(s) => {
            let Ok(val) = ctx.get_ref(&s) else {
                return Val::default();
            };
            let Val::String(path) = val else {
                return Val::default();
            };
            std::fs::read_to_string(&**path)
        }
        _ => return Val::default(),
    };
    match result {
        Ok(content) => Val::String(Str::from(content)),
        Err(err) => {
            eprintln!("{}", err);
            Val::default()
        }
    }
}
