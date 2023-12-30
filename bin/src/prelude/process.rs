use std::{
    process::Command,
    rc::Rc,
};

use airlang::{
    EvalMode,
    ListMode,
    Map,
    MutableCtx,
    Symbol,
    Val,
};
use airlang_ext::ExtFn;

use crate::prelude::{
    default_mode,
    list_mode,
    map_mode_for_some,
    put_func,
    ExtFunc,
    Prelude,
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
    let id = unsafe { Symbol::from_str_unchecked("repl.execute") };

    let mut map = Map::default();
    let program_key = Val::Symbol(unsafe { Symbol::from_str_unchecked(PROGRAM) });
    map.insert(program_key, default_mode());
    let arguments_key = Val::Symbol(unsafe { Symbol::from_str_unchecked(ARGUMENTS) });
    map.insert(arguments_key, list_mode(ListMode::Eval(EvalMode::More)));

    let input_mode = map_mode_for_some(map);
    let output_mode = default_mode();
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

    let child = Command::new(&*program).args(arguments).spawn();
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
