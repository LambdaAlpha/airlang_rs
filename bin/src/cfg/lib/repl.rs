use airlang::cfg::CfgMod;
use airlang::cfg::lib::Library;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::memo::Contract;
use airlang::semantics::memo::Memo;
use airlang::semantics::val::TextVal;
use airlang::semantics::val::Val;
use airlang::type_::Symbol;
use airlang::type_::Text;

#[derive(Clone)]
pub struct ReplLib {
    pub help: TextVal,
}

impl Default for ReplLib {
    fn default() -> Self {
        Self { help: help() }
    }
}

impl CfgMod for ReplLib {
    fn extend(self, cfg: &Cfg) {
        cfg.extend_scope(Symbol::from_str_unchecked("repl.help"), Val::Text(self.help));
    }
}

impl Library for ReplLib {
    fn prelude(&self, memo: &mut Memo) {
        let _ = memo.put(
            Symbol::from_str_unchecked("help"),
            Val::Text(self.help.clone()),
            Contract::default(),
        );
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
