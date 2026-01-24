use airlang::cfg::CfgMod;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_ID;
use airlang::semantics::val::TextVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Text;
use const_format::concatcp;

#[derive(Clone)]
pub struct ReplLib {
    pub help: TextVal,
}

const REPL: &str = "repl";

pub const HELP: &str = concatcp!(PREFIX_ID, REPL, ".help");

impl Default for ReplLib {
    fn default() -> Self {
        Self { help: help() }
    }
}

impl CfgMod for ReplLib {
    fn extend(self, cfg: &Cfg) {
        cfg.extend_scope(Key::from_str_unchecked(HELP), Val::Text(self.help));
    }
}

pub fn help() -> TextVal {
    Text::from(HELP_DOC).into()
}

const HELP_DOC: &str = r##"
keyboard shortcuts:
    Ctrl + C: exit this program
    Up/Down: switch through command history
    Alt + M: switch multiline mode

prelude:
    help: help message
    ;: call a program, i.e. `git ; .[commit --amend]`

library:
    repl.help: help message, named "help" in prelude
    command.call: call a program, named ";" in prelude
"##;
