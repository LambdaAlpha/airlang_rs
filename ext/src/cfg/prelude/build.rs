use std::env::current_dir;
use std::fs::read_to_string;
use std::path::Path;

use airlang::Air;
use airlang::cfg::prelude::FreeImpl;
use airlang::cfg::prelude::FreePrimFn;
use airlang::cfg::prelude::Prelude;
use airlang::cfg::prelude::setup::default_free_mode;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;
use airlang::syntax::parse;
use airlang::type_::Symbol;
use airlang::type_::Text;
use log::error;

#[derive(Clone)]
pub struct BuildPrelude {
    pub load: FreePrimFuncVal,
}

impl Default for BuildPrelude {
    fn default() -> Self {
        Self { load: load() }
    }
}

impl Prelude for BuildPrelude {
    fn put(self, ctx: &mut Ctx) {
        self.load.put(ctx);
    }
}

// todo rename
// todo design
pub fn load() -> FreePrimFuncVal {
    FreePrimFn { id: "build.load", f: FreeImpl::new(fn_load), mode: default_free_mode() }.free()
}

const CUR_URL_KEY: &str = "build.this_url";

fn fn_load(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(url) = input else {
        error!("input {input:?} should be a text");
        return Val::default();
    };
    let url = Text::from(url);
    let cur_url_key = Symbol::from_str_unchecked(CUR_URL_KEY);
    let cur_url = get_cur_url(cfg, cur_url_key);
    let new_url =
        cur_url.as_ref().and_then(|cur_url| join_url(cur_url, &url)).unwrap_or(String::from(url));
    load_from_url(cfg, new_url)
}

fn load_from_url(cfg: &mut Cfg, url: String) -> Val {
    let content = match read_to_string(&url) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("failed to read {url}: {err}");
            return Val::default();
        }
    };
    let Ok(val) = parse(&content) else {
        error!("{content} should be a valid air source code");
        return Val::default();
    };

    let mut mod_air = Air::new(cfg.clone());
    let cur_url_key = Symbol::from_str_unchecked(CUR_URL_KEY);
    mod_air.cfg_mut().begin_scope();
    mod_air.cfg_mut().extend_scope(cur_url_key, Val::Text(Text::from(url).into()));
    mod_air.interpret(val)
}

fn get_cur_url(cfg: &Cfg, key: Symbol) -> Option<String> {
    if let Some(val) = cfg.import(key) {
        return if let Val::Text(url) = val { Some((**url).clone()) } else { None };
    }
    let Ok(cur_dir) = current_dir() else {
        return None;
    };
    let Ok(cur_dir) = cur_dir.into_os_string().into_string() else {
        return None;
    };
    Some(cur_dir)
}

fn join_url(cur_url: &str, url: &str) -> Option<String> {
    let parent = <_ as AsRef<Path>>::as_ref(cur_url).parent()?;
    let new_url = parent.join(url).canonicalize().ok()?;
    let new_url = new_url.into_os_string().into_string().ok()?;
    Some(new_url)
}
