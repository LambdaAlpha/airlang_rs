use std::process::Command;

use airlang::{
    Bytes,
    FuncVal,
    Int,
    List,
    Map,
    Mode,
    MutableCtx,
    Str,
    Symbol,
    Transform,
    Val,
};

use crate::prelude::{
    form_mode,
    map_all_mode,
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
    let input_mode = map_all_mode(form_mode(), Mode::default(), Transform::default());
    let output_mode = Mode::default();
    named_free_fn("process.call", input_mode, output_mode, fn_call)
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

    let output = Command::new(&**program).args(arguments).output();
    let Ok(output) = output else {
        return Val::default();
    };

    let stdout = Val::Bytes(Bytes::from(output.stdout).into());
    let stderr = Val::Bytes(Bytes::from(output.stderr).into());
    let status = if let Some(status) = output.status.code() {
        Val::Int(Int::from(status).into())
    } else {
        Val::default()
    };

    let mut map = Map::default();
    let output_key = Val::Symbol(unsafe { Symbol::from_str_unchecked("output") });
    map.insert(output_key, stdout);
    let error_key = Val::Symbol(unsafe { Symbol::from_str_unchecked("error") });
    map.insert(error_key, stderr);
    let status_key = Val::Symbol(unsafe { Symbol::from_str_unchecked("status") });
    map.insert(status_key, status);
    Val::Map(map.into())
}
