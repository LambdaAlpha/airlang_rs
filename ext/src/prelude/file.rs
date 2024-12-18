use airlang::{
    ConstFnCtx,
    FuncMode,
    FuncVal,
    Mode,
    MutCtx,
    Text,
    Val,
};

use crate::prelude::{
    Named,
    Prelude,
    named_const_fn,
};

pub(crate) struct FilePrelude {
    pub(crate) read_to_text: Named<FuncVal>,
}

impl Default for FilePrelude {
    fn default() -> Self {
        Self {
            read_to_text: read_to_text(),
        }
    }
}

impl Prelude for FilePrelude {
    fn put(&self, mut ctx: MutCtx) {
        self.read_to_text.put(ctx.reborrow());
    }
}

fn read_to_text() -> Named<FuncVal> {
    let id = "file.read_to_text";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = false;
    let f = fn_read_to_text;
    named_const_fn(id, mode, cacheable, f)
}

fn fn_read_to_text(ctx: ConstFnCtx, input: Val) -> Val {
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
            eprintln!("{}", err);
            Val::default()
        }
    }
}
