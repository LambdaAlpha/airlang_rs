use airlang::prelude::Prelude;
use airlang::prelude::PreludeCtx;
use airlang_ext::prelude::StdPrelude;

use self::eval::EvalPrelude;
use self::process::ProcessPrelude;
use self::repl::ReplPrelude;

#[derive(Default)]
pub struct BinPrelude {
    pub std: StdPrelude,
    pub repl: ReplPrelude,
    pub eval: EvalPrelude,
    pub process: ProcessPrelude,
}

impl Prelude for BinPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.std.put(ctx);
        self.repl.put(ctx);
        self.eval.put(ctx);
        self.process.put(ctx);
    }
}

pub mod repl;

pub mod eval;

pub mod process;
