use std::process::Command;

use airlang::bug;
use airlang::cfg::CfgMod;
use airlang::cfg::export_func;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_CELL;
use airlang::semantics::func::CtxFreeFunc;
use airlang::semantics::val::PrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use const_format::concatcp;

#[derive(Copy, Clone)]
pub struct CmdLib {
    pub run: PrimFuncVal,
}

const COMMAND: &str = "command";

pub const RUN: &str = concatcp!(PREFIX_CELL, COMMAND, ".run");

impl Default for CmdLib {
    fn default() -> Self {
        Self { run: CtxFreeFunc { fn_: run }.build() }
    }
}

impl CfgMod for CmdLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        export_func(cfg, RUN, self.run);
    }
}

// todo rename
// todo design
// todo impl
pub fn run(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(cmd) = input else {
        return bug!(cfg, "{RUN}: expected input to be a text, but got {input}");
    };
    let cmd = &**cmd;
    let child = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(cmd).spawn()
    } else {
        Command::new("sh").arg("-c").arg(cmd).spawn()
    };
    let Ok(mut child) = child else {
        eprintln!("failed to run the command {cmd}");
        return Val::default();
    };
    let Ok(status) = child.wait() else {
        return Val::default();
    };

    if let Some(status) = status.code()
        && status != 0
    {
        println!("command exit with code: {status}");
    }
    Val::default()
}
