use std::process::Command;

use airlang::prelude::FreeFn;
use airlang::prelude::Prelude;
use airlang::prelude::PreludeCtx;
use airlang::prelude::free_impl;
use airlang::prelude::mode::CodeMode;
use airlang::prelude::mode::FuncMode;
use airlang::prelude::mode::SymbolMode;
use airlang::semantics::val::FreeStaticPrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Byte;
use airlang::type_::Int;
use airlang::type_::List;
use airlang::type_::Map;
use airlang::type_::Symbol;
use airlang::type_::Text;

pub struct ProcessPrelude {
    pub call: FreeStaticPrimFuncVal,
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
pub fn call() -> FreeStaticPrimFuncVal {
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
    let program_key = Val::Symbol(Symbol::from_str_unchecked(PROGRAM));
    let Some(Val::Text(program)) = map.remove(&program_key) else {
        return Val::default();
    };
    let arguments_key = Val::Symbol(Symbol::from_str_unchecked(ARGUMENTS));
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
    let output_key = Val::Symbol(Symbol::from_str_unchecked("output"));
    map.insert(output_key, stdout);
    let error_key = Val::Symbol(Symbol::from_str_unchecked("error"));
    map.insert(error_key, stderr);
    let status_key = Val::Symbol(Symbol::from_str_unchecked("status"));
    map.insert(status_key, status);
    Val::Map(map.into())
}
