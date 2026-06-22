use airlang::cfg::CfgMod;
use airlang::cfg::export_func;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_CELL;
use airlang::semantics::func::CtxFreeFunc;
use airlang::semantics::val::PrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Byte;
use airlang::type_::Key;
use airlang::type_::Map;
use const_format::concatcp;

#[derive(Copy, Clone)]
pub struct WasmLib {
    pub parse: PrimFuncVal,
}

const DEV_WASM: &str = "dev.wasm";

pub const PARSE: &str = concatcp!(PREFIX_CELL, DEV_WASM, ".parse");

impl Default for WasmLib {
    fn default() -> Self {
        Self { parse: CtxFreeFunc { fn_: parse }.build() }
    }
}

impl CfgMod for WasmLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        export_func(cfg, PARSE, self.parse);
    }
}

fn parse(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(text) = input else {
        panic!("{PARSE}: expected input to be a text, but got {input}");
    };
    let text: &str = &text;
    match wat::parse_str(text) {
        Ok(bytes) => Val::Byte(Byte::from(bytes).into()),
        Err(e) => panic!("{PARSE}: parse failed {e}"),
    }
}
