use airlang::cfg::CfgMod;
use airlang::semantics::cfg::Cfg;

#[derive(Copy, Clone)]
pub struct ReplLib {}

#[expect(dead_code)]
const REPL: &str = "repl";

#[expect(clippy::derivable_impls)]
impl Default for ReplLib {
    fn default() -> Self {
        Self {}
    }
}

impl CfgMod for ReplLib {
    fn extend(self, _cfg: &mut Cfg) {}
}
