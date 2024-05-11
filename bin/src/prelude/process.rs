use std::process::Command;

use airlang::{
    FuncVal,
    List,
    Map,
    MutableCtx,
    Str,
    Symbol,
    Val,
};

use crate::prelude::{
    default_mode,
    map_some_mode,
    named_free_fn,
    Named,
    Prelude,
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
    fn put(&self, mut ctx: MutableCtx) {
        self.call.put(ctx.reborrow());
    }
}

const PROGRAM: &str = "program";
const ARGUMENTS: &str = "arguments";

fn call() -> Named<FuncVal> {
    let mut map = Map::default();
    let program_key = Val::Symbol(unsafe { Symbol::from_str_unchecked(PROGRAM) });
    map.insert(program_key, default_mode());
    let arguments_key = Val::Symbol(unsafe { Symbol::from_str_unchecked(ARGUMENTS) });
    map.insert(arguments_key, default_mode());

    let input_mode = map_some_mode(map);
    let output_mode = default_mode();
    named_free_fn("repl.execute", input_mode, output_mode, fn_call)
}

fn fn_call(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let program_key = Val::Symbol(unsafe { Symbol::from_str_unchecked(PROGRAM) });
    let Some(Val::String(program)) = map.remove(&program_key) else {
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
            let Val::String(arg) = val else {
                return None;
            };
            let arg = Str::from(arg);
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
