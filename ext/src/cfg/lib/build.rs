use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use airlang::Air;
use airlang::bug;
use airlang::cfg::CfgMod;
use airlang::cfg::extend_func;
use airlang::cfg::lib::FreeImpl;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_ID;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;
use airlang::syntax::parse;
use airlang::type_::Cell;
use airlang::type_::Key;
use airlang::type_::Text;
use const_format::concatcp;

#[derive(Clone)]
pub struct BuildLib {
    pub load: FreePrimFuncVal,
}

const BUILD: &str = "build";

pub const LOAD: &str = concatcp!(PREFIX_ID, BUILD, ".load");

impl Default for BuildLib {
    fn default() -> Self {
        Self { load: load() }
    }
}

impl CfgMod for BuildLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, LOAD, self.load);
    }
}

// todo rename
// todo design
pub fn load() -> FreePrimFuncVal {
    FreeImpl { free: fn_load }.build()
}

// todo rename
const CUR_URL_KEY: &str = "build.this_url";

fn fn_load(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(url) = input else {
        return bug!(cfg, "{LOAD}: expected input to be a text, but got {input:?}");
    };
    let url = Text::from(url);
    let cur_url_key = Key::from_str_unchecked(CUR_URL_KEY);
    let cur_url = get_cur_url(cfg, cur_url_key);
    let new_url =
        cur_url.as_ref().and_then(|cur_url| join_url(cur_url, &url)).unwrap_or(String::from(url));
    load_from_url(cfg, new_url)
}

fn load_from_url(cfg: &mut Cfg, url: String) -> Val {
    let mut buffer = String::new();
    let content = match read_to_string(&url, &mut buffer) {
        Ok(content) => content,
        Err(_err) => {
            return Val::Key(Key::from_str_unchecked("_read_error"));
        }
    };
    let Ok(val) = parse(content) else {
        return Val::Key(Key::from_str_unchecked("_parse_error"));
    };

    let Some(mut mod_air) = Air::new(cfg.clone()) else {
        return Val::default();
    };
    let cur_url_key = Key::from_str_unchecked(CUR_URL_KEY);
    mod_air.cfg_mut().begin_scope();
    mod_air.cfg_mut().extend_scope(cur_url_key, Val::Text(Text::from(url).into()));
    let output = mod_air.interpret(val);
    Val::Cell(Cell::new(output).into())
}

fn read_to_string<'a>(url: &str, buffer: &'a mut String) -> std::io::Result<&'a str> {
    let mut file = File::open(url)?;
    file.read_to_string(buffer)?;
    // remove bom
    let content = buffer.strip_prefix('\u{feff}').unwrap_or(&**buffer);
    Ok(content)
}

fn get_cur_url(cfg: &Cfg, key: Key) -> Option<String> {
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
