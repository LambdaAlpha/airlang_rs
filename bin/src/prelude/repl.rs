use airlang::{
    FuncMode,
    FuncVal,
    PreludeCtx,
    Val,
};

use crate::prelude::{
    Named,
    Prelude,
    named_free_fn,
};

pub(crate) struct ReplPrelude {
    pub(crate) help: Named<FuncVal>,
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

fn help() -> Named<FuncVal> {
    let id = "help";
    let f = fn_help;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
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
