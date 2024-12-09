use airlang::{
    FuncVal,
    Mode,
    MutCtx,
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
    fn put(&self, mut ctx: MutCtx) {
        self.help.put(ctx.reborrow());
    }
}

fn help() -> Named<FuncVal> {
    let id = "help";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = false;
    let f = fn_help;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

const HELP_DOC: &str = "\
functions:
    help: display this message

keyboard shortcuts:
    Ctrl + C: exit this program
    Up/Down: switch through command history
    Alt + M: switch multiline mode
";

fn fn_help(_input: Val) -> Val {
    print!("{HELP_DOC}");
    Val::default()
}
