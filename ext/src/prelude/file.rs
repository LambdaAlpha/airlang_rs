use airlang::ConstRef;
use airlang::Ctx;
use airlang::FuncMode;
use airlang::FuncVal;
use airlang::PreludeCtx;
use airlang::Text;
use airlang::Val;

use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::const_impl;
use crate::prelude::named_const_fn;

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
    let f = const_impl(fn_read_to_text);
    let mode = FuncMode::default();
    named_const_fn(id, f, mode)
}

fn fn_read_to_text(ctx: ConstRef<Ctx>, input: Val) -> Val {
    let result = match input {
        Val::Text(path) => std::fs::read_to_string(&**path),
        Val::Symbol(s) => {
            let Ok(val) = ctx.get_ref(s) else {
                return Val::default();
            };
            let Val::Text(path) = val else {
                return Val::default();
            };
            std::fs::read_to_string(&***path)
        }
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
