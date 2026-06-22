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
use airlang::type_::Text;
use const_format::concatcp;

#[derive(Copy, Clone)]
pub struct FileLib {
    pub read_to_text: PrimFuncVal,
}

const FILE: &str = "file";

pub const READ_TO_TEXT: &str = concatcp!(PREFIX_CELL, FILE, ".read_to_text");

impl Default for FileLib {
    fn default() -> Self {
        Self { read_to_text: CtxFreeFunc { fn_: read_to_text }.build() }
    }
}

impl CfgMod for FileLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        export_func(cfg, READ_TO_TEXT, self.read_to_text);
    }
}

pub fn read_to_text(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(path) = input else {
        return bug!(cfg, "{READ_TO_TEXT}: expected input to be a text, but got {input}");
    };
    let result = std::fs::read_to_string(&**path);
    match result {
        Ok(content) => Val::Text(Text::from(content).into()),
        Err(err) => {
            eprintln!("{err}");
            Val::default()
        },
    }
}
