use std::process::Command;

use airlang::cfg::CfgMod;
use airlang::cfg::CoreCfg;
use airlang::cfg::exception::fail;
use airlang::cfg::exception::illegal_input;
use airlang::cfg::lib::FreePrimFn;
use airlang::cfg::lib::adapter::CallPrimAdapter;
use airlang::cfg::lib::adapter::SymbolAdapter;
use airlang::cfg::lib::adapter::prim_adapter;
use airlang::cfg::lib::free_impl;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;
use log::error;

#[derive(Clone)]
pub struct CmdLib {
    pub call: FreePrimFuncVal,
}

impl Default for CmdLib {
    fn default() -> Self {
        Self { call: call() }
    }
}

impl CfgMod for CmdLib {
    fn extend(self, cfg: &Cfg) {
        let call_adapter = prim_adapter(SymbolAdapter::Literal, CallPrimAdapter::Eval);
        CoreCfg::extend_adapter(cfg, &self.call.id(), call_adapter);
        self.call.extend(cfg);
    }
}

// todo rename
// todo design
// todo impl
pub fn call() -> FreePrimFuncVal {
    FreePrimFn { id: "command.call", f: free_impl(fn_call) }.free()
}

fn fn_call(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let program: &str = match &pair.first {
        Val::Text(program) => program,
        Val::Symbol(symbol) => symbol,
        v => {
            error!("program {v:?} should be a text or a symbol");
            return illegal_input(cfg);
        }
    };
    let Val::List(arguments) = &pair.second else {
        error!("arguments {:?} should be a list", pair.second);
        return illegal_input(cfg);
    };
    let arguments = arguments
        .iter()
        .map(|val| {
            let arg: &str = match val {
                Val::Text(text) => text,
                Val::Symbol(symbol) => symbol,
                v => {
                    error!("argument {v:?} should be a text or a symbol");
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
        return fail(cfg);
    };
    let Ok(status) = child.wait() else {
        return fail(cfg);
    };

    if let Some(status) = status.code()
        && status != 0
    {
        println!("program exit with code: {status}");
    }
    Val::default()
}
