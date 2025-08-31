use airlang::cfg::CfgMod;
use airlang::cfg::lib::FreePrimFn;
use airlang::cfg::lib::Library;
use airlang::cfg::lib::free_impl;
use airlang::cfg::lib::setup::default_free_mode;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;

#[derive(Clone)]
pub struct ReplLib {
    pub help: FreePrimFuncVal,
}

impl Default for ReplLib {
    fn default() -> Self {
        Self { help: help() }
    }
}

impl CfgMod for ReplLib {
    fn extend(self, cfg: &Cfg) {
        self.help.extend(cfg);
    }
}

impl Library for ReplLib {
    fn prelude(&self, ctx: &mut Ctx) {
        self.help.prelude(ctx);
    }
}

// todo design
pub fn help() -> FreePrimFuncVal {
    FreePrimFn { id: "help", f: free_impl(fn_help), mode: default_free_mode() }.free()
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

fn fn_help(_cfg: &mut Cfg, _input: Val) -> Val {
    print!("{HELP_DOC}");
    Val::default()
}
