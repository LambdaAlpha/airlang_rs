use std::rc::Rc;

use airlang::{
    CtxForConstFn,
    MutableCtx,
    Str,
    Symbol,
    Val,
};

use crate::{
    prelude::{
        default_mode,
        put_func,
        symbol_value_mode,
        ExtFunc,
        Prelude,
    },
    ExtFn,
};

pub(crate) struct FilePrelude {
    pub(crate) read_to_string: Rc<ExtFunc>,
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
        put_func(&self.read_to_string, ctx.reborrow());
    }
}

fn read_to_string() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("file.read_to_string") };
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
    let ext_fn = ExtFn::new_const(fn_read_to_string);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
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
