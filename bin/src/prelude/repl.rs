use airlang::prelude::FreeFn;
use airlang::prelude::Prelude;
use airlang::prelude::PreludeCtx;
use airlang::prelude::free_impl;
use airlang::prelude::mode::FuncMode;
use airlang::semantics::val::FreeStaticPrimFuncVal;
use airlang::semantics::val::Val;

pub struct ReplPrelude {
    pub help: FreeStaticPrimFuncVal,
}

impl Default for ReplPrelude {
    fn default() -> Self {
        Self { help: help() }
    }
}

impl Prelude for ReplPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.help.put(ctx);
    }
}

// todo design
pub fn help() -> FreeStaticPrimFuncVal {
    FreeFn { id: "help", f: free_impl(fn_help), mode: FuncMode::default() }.free_static()
}

const HELP_DOC: &str = "\
functions:
    help: display this message
    $: call a program, i.e. `git $ [status]`
    repl.reset: reset the repl context to its initial state

keyboard shortcuts:
    Ctrl + C: exit this program
    Up/Down: switch through command history
    Alt + M: switch multiline mode
";

fn fn_help(_input: Val) -> Val {
    print!("{HELP_DOC}");
    Val::default()
}
