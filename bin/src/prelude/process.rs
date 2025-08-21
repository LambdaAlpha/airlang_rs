use std::process::Command;

use airlang::prelude::FreePrimFn;
use airlang::prelude::Prelude;
use airlang::prelude::free_impl;
use airlang::prelude::mode::FuncMode;
use airlang::prelude::mode::SymbolMode;
use airlang::prelude::mode::TaskPrimMode;
use airlang::prelude::setup::free_mode;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;
use log::error;

pub struct ProcessPrelude {
    pub call: FreePrimFuncVal,
}

impl Default for ProcessPrelude {
    fn default() -> Self {
        Self { call: call() }
    }
}

impl Prelude for ProcessPrelude {
    fn put(self, ctx: &mut Ctx) {
        self.call.put(ctx);
    }
}

// todo rename
// todo design
// todo impl
pub fn call() -> FreePrimFuncVal {
    FreePrimFn {
        id: "$",
        f: free_impl(fn_call),
        mode: free_mode(FuncMode::prim_mode(SymbolMode::Literal, TaskPrimMode::Eval)),
    }
    .free()
}

fn fn_call(input: Val) -> Val {
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
