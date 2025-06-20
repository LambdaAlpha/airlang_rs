use airlang::prelude::FreeFn;
use airlang::prelude::Prelude;
use airlang::prelude::PreludeCtx;
use airlang::prelude::free_impl;
use airlang::prelude::mode::FuncMode;
use airlang::semantics::val::FreeStaticPrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Text;

pub struct FilePrelude {
    pub read_to_text: FreeStaticPrimFuncVal,
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

pub fn read_to_text() -> FreeStaticPrimFuncVal {
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
