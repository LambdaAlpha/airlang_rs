use airlang::prelude::FreePrimFn;
use airlang::prelude::Prelude;
use airlang::prelude::PreludeCtx;
use airlang::prelude::free_impl;
use airlang::prelude::setup::default_free_mode;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Text;
use log::error;

pub struct FilePrelude {
    pub read_to_text: FreePrimFuncVal,
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

pub fn read_to_text() -> FreePrimFuncVal {
    FreePrimFn { id: "file.read_to_text", f: free_impl(fn_read_to_text), mode: default_free_mode() }
        .free()
}

fn fn_read_to_text(input: Val) -> Val {
    let result = match input {
        Val::Text(path) => std::fs::read_to_string(&**path),
        v => {
            error!("input {v:?} should be a text");
            return Val::default();
        }
    };
    match result {
        Ok(content) => Val::Text(Text::from(content).into()),
        Err(err) => {
            eprintln!("{err}");
            Val::default()
        }
    }
}
