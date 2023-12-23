use std::{
    process::Command,
    rc::Rc,
};

use airlang::{
    Bytes,
    EvalMode,
    Int,
    IoMode,
    Map,
    MapVal,
    MutableCtx,
    Symbol,
    Val,
};

use crate::{
    prelude::{
        put_func,
        ExtFunc,
        Prelude,
    },
    ExtFn,
};

pub(crate) struct ProcessPrelude {
    pub(crate) call: Rc<ExtFunc>,
}

impl Default for ProcessPrelude {
    fn default() -> Self {
        Self { call: call() }
    }
}

impl Prelude for ProcessPrelude {
    fn put(&self, mut ctx: MutableCtx) {
        put_func(&self.call, ctx.reborrow());
    }
}

const PROGRAM: &str = "program";
const ARGUMENTS: &str = "arguments";

fn call() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("process.call") };

    let mut map = Map::default();
    let program_key = Val::Symbol(unsafe { Symbol::from_str_unchecked(PROGRAM) });
    map.insert(program_key, IoMode::Any(EvalMode::More));
    let arguments_key = Val::Symbol(unsafe { Symbol::from_str_unchecked(ARGUMENTS) });
    map.insert(arguments_key, IoMode::List(EvalMode::More));

    let input_mode = IoMode::MapForSome(map);
    let output_mode = IoMode::Any(EvalMode::More);
    let ext_fn = ExtFn::new_free(fn_call);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
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
    let arguments = arguments
        .into_iter()
        .map(|val| {
            let Val::String(arg) = val else {
                return None;
            };
            Some(String::from(arg))
        })
        .try_collect::<Vec<String>>();
    let Some(arguments) = arguments else {
        return Val::default();
    };

    let output = Command::new(&*program).args(arguments).output();
    let Ok(output) = output else {
        return Val::default();
    };

    let stdout = Val::Bytes(Bytes::from(output.stdout));
    let stderr = Val::Bytes(Bytes::from(output.stderr));
    let status = if let Some(status) = output.status.code() {
        Val::Int(Int::from(status))
    } else {
        Val::default()
    };

    let mut map = MapVal::default();
    let output_key = Val::Symbol(unsafe { Symbol::from_str_unchecked("output") });
    map.insert(output_key, stdout);
    let error_key = Val::Symbol(unsafe { Symbol::from_str_unchecked("error") });
    map.insert(error_key, stderr);
    let status_key = Val::Symbol(unsafe { Symbol::from_str_unchecked("status") });
    map.insert(status_key, status);
    Val::Map(map)
}
