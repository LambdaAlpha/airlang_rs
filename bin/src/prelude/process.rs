use std::process::Command;

use airlang::prelude::FreeFn;
use airlang::prelude::Prelude;
use airlang::prelude::PreludeCtx;
use airlang::prelude::free_impl;
use airlang::semantics::func::FuncMode;
use airlang::semantics::mode::CodeMode;
use airlang::semantics::mode::SymbolMode;
use airlang::semantics::val::FreeStaticPrimFuncVal;
use airlang::semantics::val::Val;

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

// todo design
// todo impl
pub fn call() -> FreeStaticPrimFuncVal {
    let forward = FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form);
    FreeFn {
        id: "$",
        f: free_impl(fn_call),
        mode: FuncMode { forward, reverse: FuncMode::default_mode() },
    }
    .free_static()
}

fn fn_call(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let program: &str = match &pair.first {
        Val::Text(program) => program,
        Val::Symbol(symbol) => symbol,
        _ => return Val::default(),
    };
    let Val::List(arguments) = &pair.second else {
        return Val::default();
    };
    let arguments = arguments
        .iter()
        .map(|val| {
            let arg: &str = match val {
                Val::Text(text) => text,
                Val::Symbol(symbol) => symbol,
                _ => return None,
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

    if let Some(status) = status.code() {
        if status != 0 {
            println!("program exit with code: {status}");
        }
    }
    Val::default()
}
