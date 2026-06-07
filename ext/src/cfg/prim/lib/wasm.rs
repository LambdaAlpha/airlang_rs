use airlang::cfg::CfgMod;
use airlang::cfg::extend_func;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_CELL;
use airlang::semantics::func::CtxFreeFunc;
use airlang::semantics::val::PrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Byte;
use const_format::concatcp;

use self::repr::parse_module;

/// see ./wasm/wasm-grammar.air
#[derive(Copy, Clone)]
pub struct WasmLib {
    pub make: PrimFuncVal,
}

const WASM: &str = "wasm";

pub const MAKE: &str = concatcp!(PREFIX_CELL, WASM, ".make");

impl CfgMod for WasmLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, MAKE, self.make);
    }
}

impl Default for WasmLib {
    fn default() -> Self {
        Self { make: CtxFreeFunc { fn_: make }.build() }
    }
}

pub fn make(_cfg: &mut Cfg, input: Val) -> Val {
    let Some(module) = parse_module(&input) else {
        return Val::default();
    };
    let bytes = module.finish();
    Val::Byte(Byte::from(bytes).into())
}

mod repr;
