use airlang::cfg::CfgMod;
use airlang::cfg::lib::FreePrimFn;
use airlang::cfg::lib::free_impl;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Text;
use log::error;

#[derive(Clone)]
pub struct FileLib {
    pub read_to_text: FreePrimFuncVal,
}

impl Default for FileLib {
    fn default() -> Self {
        Self { read_to_text: read_to_text() }
    }
}

impl CfgMod for FileLib {
    fn extend(self, cfg: &Cfg) {
        self.read_to_text.extend(cfg);
    }
}

pub fn read_to_text() -> FreePrimFuncVal {
    FreePrimFn { id: "file.read_to_text", f: free_impl(fn_read_to_text) }.free()
}

fn fn_read_to_text(_cfg: &mut Cfg, input: Val) -> Val {
    let result = match input {
        Val::Text(path) => std::fs::read_to_string(&**path),
        v => {
            error!("input {v:?} should be a text");
            return Val::default();
        }
    };
    match result {
        Ok(content) => Val::Text(Text::from(content).into()),
        Err(err) => {
            eprintln!("{err}");
            Val::default()
        }
    }
}
