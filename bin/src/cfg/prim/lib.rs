use airlang::cfg::CfgMod;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use airlang_ext::cfg::prim::lib::ExtPrimLib;

use self::cmd::CmdLib;
use self::repl::ReplLib;

#[derive(Default, Copy, Clone)]
pub struct BinPrimLib {
    pub repl: ReplLib,
    pub cmd: CmdLib,
    pub ext: ExtPrimLib,
}

impl CfgMod for BinPrimLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        self.repl.export(cfg);
        self.cmd.export(cfg);
        self.ext.export(cfg);
    }
}

pub mod repl;

pub mod cmd;
