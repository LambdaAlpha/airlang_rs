use airlang::FreeStaticPrimFuncVal;
use airlang::FuncMode;
use airlang::PreludeCtx;
use airlang::Text;
use airlang::Val;

use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::free_impl;

pub(crate) struct FilePrelude {
    pub(crate) read_to_text: FreeStaticPrimFuncVal,
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

fn read_to_text() -> FreeStaticPrimFuncVal {
    FreeFn { id: "file.read_to_text", f: free_impl(fn_read_to_text), mode: FuncMode::default() }
        .free_static()
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
