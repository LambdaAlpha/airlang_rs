use std::process::Command;

use airlang::{
    FuncVal,
    List,
    Map,
    Mode,
    MutCtx,
    PrimitiveMode,
    Symbol,
    Text,
    Val,
};

use crate::prelude::{
    Named,
    Prelude,
    form_mode,
    map_mode,
    named_static_fn,
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

const PROGRAM: &str = "program";
const ARGUMENTS: &str = "arguments";

fn call() -> Named<FuncVal> {
    let call_mode = map_mode(
        Map::default(),
        form_mode(),
        Mode::default(),
        PrimitiveMode::default(),
    );
    let ask_mode = Mode::default();
    named_static_fn("repl.execute", call_mode, ask_mode, false, fn_call)
}

fn fn_call(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let program_key = Val::Symbol(unsafe { Symbol::from_str_unchecked(PROGRAM) });
    let Some(Val::Text(program)) = map.remove(&program_key) else {
        return Val::default();
    };
    let arguments_key = Val::Symbol(unsafe { Symbol::from_str_unchecked(ARGUMENTS) });
    let Some(Val::List(arguments)) = map.remove(&arguments_key) else {
        return Val::default();
    };
    let arguments = List::from(arguments);
    let arguments = arguments
        .into_iter()
        .map(|val| {
            let Val::Text(arg) = val else {
                return None;
            };
            let arg = Text::from(arg);
            Some(String::from(arg))
        })
        .collect::<Option<Vec<String>>>();
    let Some(arguments) = arguments else {
        return Val::default();
    };

    let child = Command::new(&**program).args(arguments).spawn();
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
