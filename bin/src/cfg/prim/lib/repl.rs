use airlang::cfg::CfgMod;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;

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
    fn export(self, _cfg: &mut Map<Key, Val>) {}
}
