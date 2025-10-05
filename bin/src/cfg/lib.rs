use airlang::cfg::CfgMod;
use airlang::semantics::cfg::Cfg;
use airlang_ext::cfg::lib::StdLib;

use self::cmd::CmdLib;
use self::repl::ReplLib;

#[derive(Default, Clone)]
pub struct BinLib {
    pub repl: ReplLib,
    pub cmd: CmdLib,
    pub std: StdLib,
}

impl CfgMod for BinLib {
    fn extend(self, cfg: &Cfg) {
        self.repl.extend(cfg);
        self.cmd.extend(cfg);
        self.std.extend(cfg);
    }
}

pub mod repl;

pub mod cmd;
