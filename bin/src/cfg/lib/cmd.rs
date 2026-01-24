use std::process::Command;

use airlang::cfg::CfgMod;
use airlang::cfg::error::illegal_input;
use airlang::cfg::extend_func;
use airlang::cfg::lib::FreeImpl;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_ID;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;
use const_format::concatcp;
use log::error;

#[derive(Clone)]
pub struct CmdLib {
    pub call: FreePrimFuncVal,
}

const COMMAND: &str = "command";

pub const CALL: &str = concatcp!(PREFIX_ID, COMMAND, ".call");

impl Default for CmdLib {
    fn default() -> Self {
        Self { call: call() }
    }
}

impl CfgMod for CmdLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, CALL, self.call);
    }
}

// todo rename
// todo design
// todo impl
pub fn call() -> FreePrimFuncVal {
    FreeImpl { free: fn_call }.build()
}

fn fn_call(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let program: &str = match &pair.left {
        Val::Text(program) => program,
        Val::Key(key) => key,
        v => {
            error!("program {v:?} should be a text or a key");
            return illegal_input(cfg);
        }
    };
    let Val::List(arguments) = &pair.right else {
        error!("arguments {:?} should be a list", pair.right);
        return illegal_input(cfg);
    };
    let arguments = arguments
        .iter()
        .map(|val| {
            let arg: &str = match val {
                Val::Text(text) => text,
                Val::Key(key) => key,
                v => {
                    error!("argument {v:?} should be a text or a key");
                    return None;
                }
            };
            Some(arg)
        })
        .collect::<Option<Vec<&str>>>();
    let Some(arguments) = arguments else {
        return illegal_input(cfg);
    };

    let child = Command::new(program).args(arguments).spawn();
    let Ok(mut child) = child else {
        eprintln!("failed to execute program");
        return Val::default();
    };
    let Ok(status) = child.wait() else {
        return Val::default();
    };

    if let Some(status) = status.code()
        && status != 0
    {
        println!("program exit with code: {status}");
    }
    Val::default()
}
