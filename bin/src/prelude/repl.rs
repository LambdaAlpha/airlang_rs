use airlang::{
    FuncVal,
    Mode,
    MutCtx,
    Val,
};

use crate::prelude::{
    named_static_fn,
    Named,
    Prelude,
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
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_static_fn("help", call_mode, ask_mode, false, fn_help)
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
