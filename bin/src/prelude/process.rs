use std::process::Command;

use airlang::{
    Form,
    FuncMode,
    FuncVal,
    Mode,
    MutCtx,
    Val,
};

use crate::prelude::{
    Named,
    Prelude,
    form_mode,
    named_free_fn,
};

pub(crate) struct ProcessPrelude {
    pub(crate) call: Named<FuncVal>,
}

impl Default for ProcessPrelude {
    fn default() -> Self {
        Self { call: call() }
    }
}

impl Prelude for ProcessPrelude {
    fn put(&self, mut ctx: MutCtx) {
        self.call.put(ctx.reborrow());
    }
}

fn call() -> Named<FuncVal> {
    let id = "$";
    let f = fn_call;
    let call = form_mode(Form::Literal);
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = false;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_call(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let program: &str = match &pair.first {
        Val::Text(program) => program,
        Val::Symbol(symbol) => symbol,
        _ => return Val::default(),
    };
    let Val::List(arguments) = &pair.second else {
        return Val::default();
    };
    let arguments = arguments
        .iter()
        .map(|val| {
            let arg: &str = match val {
                Val::Text(text) => text,
                Val::Symbol(symbol) => symbol,
                _ => return None,
            };
            Some(arg)
        })
        .collect::<Option<Vec<&str>>>();
    let Some(arguments) = arguments else {
        return Val::default();
    };

    let child = Command::new(program).args(arguments).spawn();
    let Ok(mut child) = child else {
        eprintln!("failed to execute program");
        return Val::default();
    };
    let Ok(status) = child.wait() else {
        return Val::default();
    };

    if let Some(status) = status.code() {
        if status != 0 {
            println!("program exit with code: {status}");
        }
    }
    Val::default()
}
