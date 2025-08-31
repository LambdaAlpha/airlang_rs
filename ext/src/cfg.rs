use airlang::cfg::CfgMod;
use airlang::cfg::CoreCfg;
use airlang::cfg::lib::Library;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::val::Val;
use airlang::type_::Symbol;

use self::lib::StdLib;

#[derive(Clone)]
pub struct StdCfg {
    pub lib: StdLib,
    pub prelude: Ctx,
}

impl Default for StdCfg {
    fn default() -> Self {
        let lib = StdLib::default();
        let mut prelude = Ctx::default();
        lib.prelude(&mut prelude);
        Self { lib, prelude }
    }
}

impl CfgMod for StdCfg {
    fn extend(self, cfg: &Cfg) {
        self.lib.extend(cfg);
        cfg.extend_scope(
            Symbol::from_str_unchecked(CoreCfg::PRELUDE),
            Val::Ctx(self.prelude.into()),
        );
    }
}

pub mod lib;
