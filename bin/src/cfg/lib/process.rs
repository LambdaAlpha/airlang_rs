use std::process::Command;

use airlang::cfg::CfgMod;
use airlang::cfg::lib::FreePrimFn;
use airlang::cfg::lib::Library;
use airlang::cfg::lib::free_impl;
use airlang::cfg::mode::CallPrimMode;
use airlang::cfg::mode::FuncMode;
use airlang::cfg::mode::SymbolMode;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;
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
    fn prelude(&self, ctx: &mut Ctx) {
        self.call.prelude(ctx);
    }
}

// todo rename
// todo design
// todo impl
pub fn call() -> FreePrimFuncVal {
    FreePrimFn {
        id: "$",
        f: free_impl(fn_call),
        mode: FuncMode::prim_mode(SymbolMode::Literal, CallPrimMode::Eval),
    }
    .free()
}

fn fn_call(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let program: &str = match &pair.first {
        Val::Text(program) => program,
        Val::Symbol(symbol) => symbol,
        v => {
            error!("program {v:?} should be a text or a symbol");
            return Val::default();
        }
    };
    let Val::List(arguments) = &pair.second else {
        error!("arguments {:?} should be a list", pair.second);
        return Val::default();
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
        return Val::default();
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
