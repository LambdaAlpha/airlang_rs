use std::process::Command;

use airlang::Byte;
use airlang::CodeMode;
use airlang::FreeStaticPrimFuncVal;
use airlang::FuncMode;
use airlang::Int;
use airlang::List;
use airlang::Map;
use airlang::PreludeCtx;
use airlang::Symbol;
use airlang::SymbolMode;
use airlang::Text;
use airlang::Val;

use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::free_impl;

pub(crate) struct ProcessPrelude {
    pub(crate) call: FreeStaticPrimFuncVal,
}

impl Default for ProcessPrelude {
    fn default() -> Self {
        Self { call: call() }
    }
}

impl Prelude for ProcessPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.call.put(ctx);
    }
}

const PROGRAM: &str = "program";
const ARGUMENTS: &str = "arguments";

// todo design
// todo impl
fn call() -> FreeStaticPrimFuncVal {
    let forward = FuncMode::map_mode(
        Map::default(),
        FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form),
        FuncMode::default_mode(),
    );
    FreeFn {
        id: "process.call",
        f: free_impl(fn_call),
        mode: FuncMode { forward, reverse: FuncMode::default_mode() },
    }
    .free_static()
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
