use std::process::Command;

use airlang::{
    Byte,
    FuncVal,
    Int,
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

const PROGRAM: &str = "program";
const ARGUMENTS: &str = "arguments";

fn call() -> Named<FuncVal> {
    let id = "process.call";
    let call_mode = map_mode(
        Map::default(),
        form_mode(),
        Mode::default(),
        PrimitiveMode::default(),
    );
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = false;
    let f = fn_call;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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

    let output = Command::new(&**program).args(arguments).output();
    let Ok(output) = output else {
        return Val::default();
    };

    let stdout = Val::Byte(Byte::from(output.stdout).into());
    let stderr = Val::Byte(Byte::from(output.stderr).into());
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
