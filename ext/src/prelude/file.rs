use airlang::FuncMode;
use airlang::FuncVal;
use airlang::PreludeCtx;
use airlang::Text;
use airlang::Val;

use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::free_impl;
use crate::prelude::named_free_fn;

pub(crate) struct FilePrelude {
    pub(crate) read_to_text: Named<FuncVal>,
}

impl Default for FilePrelude {
    fn default() -> Self {
        Self { read_to_text: read_to_text() }
    }
}

impl Prelude for FilePrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.read_to_text.put(ctx);
    }
}

fn read_to_text() -> Named<FuncVal> {
    let id = "file.read_to_text";
    let f = free_impl(fn_read_to_text);
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_read_to_text(input: Val) -> Val {
    let result = match input {
        Val::Text(path) => std::fs::read_to_string(&**path),
        _ => return Val::default(),
    };
    match result {
        Ok(content) => Val::Text(Text::from(content).into()),
        Err(err) => {
            eprintln!("{err}");
            Val::default()
        }
    }
}
