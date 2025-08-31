use airlang::cfg::CfgMod;
use airlang::cfg::lib::Library;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::ctx::Ctx;
use airlang_ext::cfg::lib::StdLib;

use self::eval::EvalLib;
use self::process::ProcessLib;
use self::repl::ReplLib;

#[derive(Default, Clone)]
pub struct BinLib {
    pub std: StdLib,
    pub repl: ReplLib,
    pub eval: EvalLib,
    pub process: ProcessLib,
}

impl CfgMod for BinLib {
    fn extend(self, cfg: &Cfg) {
        self.std.extend(cfg);
        self.repl.extend(cfg);
        self.eval.extend(cfg);
        self.process.extend(cfg);
    }
}

impl Library for BinLib {
    fn prelude(&self, ctx: &mut Ctx) {
        self.std.prelude(ctx);
        self.repl.prelude(ctx);
        self.eval.prelude(ctx);
        self.process.prelude(ctx);
    }
}

pub mod repl;

pub mod eval;

pub mod process;
