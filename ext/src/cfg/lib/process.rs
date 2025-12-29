use std::process::Command;

use airlang::cfg::CfgMod;
use airlang::cfg::exception::fail;
use airlang::cfg::exception::illegal_input;
use airlang::cfg::extend_func;
use airlang::cfg::lib::FreePrimFn;
use airlang::cfg::lib::free_impl;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Byte;
use airlang::type_::Int;
use airlang::type_::Key;
use airlang::type_::List;
use airlang::type_::Map;
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
        extend_func(cfg, "_process.call", self.call);
    }
}

const PROGRAM: &str = "program";
const ARGUMENTS: &str = "arguments";

// todo design
// todo impl
pub fn call() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_call) }.free()
}

fn fn_call(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Map(mut map) = input else {
        error!("input {input:?} should be a map");
        return illegal_input(cfg);
    };
    let Some(program) = map.remove(&Key::from_str_unchecked(PROGRAM)) else {
        error!("program name should be provided");
        return illegal_input(cfg);
    };
    let Val::Text(program) = program else {
        error!("program {program:?} should be a text");
        return illegal_input(cfg);
    };
    let Some(arguments) = map.remove(&Key::from_str_unchecked(ARGUMENTS)) else {
        error!("arguments should be provided");
        return illegal_input(cfg);
    };
    let Val::List(arguments) = arguments else {
        error!("arguments {arguments:?} should be a list");
        return illegal_input(cfg);
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
        return illegal_input(cfg);
    };

    let output = Command::new(&**program).args(arguments).output();
    let output = match output {
        Ok(output) => output,
        Err(e) => {
            error!("call program failed: {e}");
            return fail(cfg);
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
    map.insert(Key::from_str_unchecked("output"), stdout);
    map.insert(Key::from_str_unchecked("error"), stderr);
    map.insert(Key::from_str_unchecked("status"), status);
    Val::Map(map.into())
}
