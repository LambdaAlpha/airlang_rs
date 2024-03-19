use airlang::{
    CtxForConstFn,
    FuncVal,
    MutableCtx,
    Str,
    Val,
};

use crate::prelude::{
    default_mode,
    named_const_fn,
    symbol_id_mode,
    Named,
    Prelude,
};

pub(crate) struct FilePrelude {
    pub(crate) read_to_string: Named<FuncVal>,
}

impl Default for FilePrelude {
    fn default() -> Self {
        Self {
            read_to_string: read_to_string(),
        }
    }
}

impl Prelude for FilePrelude {
    fn put(&self, mut ctx: MutableCtx) {
        self.read_to_string.put(ctx.reborrow());
    }
}

fn read_to_string() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn(
        "file.read_to_string",
        input_mode,
        output_mode,
        fn_read_to_string,
    )
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
