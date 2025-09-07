use std::process::Command;

use airlang::cfg::CfgMod;
use airlang::cfg::lib::FreePrimFn;
use airlang::cfg::lib::Library;
use airlang::cfg::lib::free_impl;
use airlang::cfg::mode::FuncMode;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::memo::Memo;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Byte;
use airlang::type_::Int;
use airlang::type_::List;
use airlang::type_::Map;
use airlang::type_::Symbol;
use airlang::type_::Text;
use log::error;

#[derive(Clone)]
pub struct ProcessLib {
    pub call: FreePrimFuncVal,
}

impl Default for ProcessLib {
    fn default() -> Self {
        Self { call: call() }
    }
}

impl CfgMod for ProcessLib {
    fn extend(self, cfg: &Cfg) {
        self.call.extend(cfg);
    }
}

impl Library for ProcessLib {
    fn prelude(&self, _memo: &mut Memo) {}
}

const PROGRAM: &str = "program";
const ARGUMENTS: &str = "arguments";

// todo design
// todo impl
pub fn call() -> FreePrimFuncVal {
    FreePrimFn { id: "process.call", f: free_impl(fn_call), mode: FuncMode::default_mode() }.free()
}

fn fn_call(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Map(mut map) = input else {
        error!("input {input:?} should be a map");
        return Val::default();
    };
    let program_key = Val::Symbol(Symbol::from_str_unchecked(PROGRAM));
    let Some(program) = map.remove(&program_key) else {
        error!("program name should be provided");
        return Val::default();
    };
    let Val::Text(program) = program else {
        error!("program {program:?} should be a text");
        return Val::default();
    };
    let arguments_key = Val::Symbol(Symbol::from_str_unchecked(ARGUMENTS));
    let Some(arguments) = map.remove(&arguments_key) else {
        error!("arguments should be provided");
        return Val::default();
    };
    let Val::List(arguments) = arguments else {
        error!("arguments {arguments:?} should be a list");
        return Val::default();
    };
    let arguments = List::from(arguments);
    let arguments = arguments
        .into_iter()
        .map(|val| {
            let Val::Text(arg) = val else {
                error!("argument {val:?} should be a text");
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
    let output = match output {
        Ok(output) => output,
        Err(e) => {
            error!("call program failed: {e}");
            return Val::default();
        }
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
