use airlang::cfg::CfgMod;
use airlang::semantics::cfg::Cfg;
use airlang_ext::cfg::prim::lib::ExtPrimLib;

use self::cmd::CmdLib;
use self::repl::ReplLib;

#[derive(Default, Clone)]
pub struct BinPrimLib {
    pub repl: ReplLib,
    pub cmd: CmdLib,
    pub ext: ExtPrimLib,
}

impl CfgMod for BinPrimLib {
    fn extend(self, cfg: &mut Cfg) {
        self.repl.extend(cfg);
        self.cmd.extend(cfg);
        self.ext.extend(cfg);
    }
}

pub mod repl;

pub mod cmd;
