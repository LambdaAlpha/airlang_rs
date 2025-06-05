use airlang::FreeStaticPrimFuncVal;
use airlang::FuncMode;
use airlang::PreludeCtx;
use airlang::Val;

use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::free_impl;

pub(crate) struct ReplPrelude {
    pub(crate) help: FreeStaticPrimFuncVal,
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
fn help() -> FreeStaticPrimFuncVal {
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
