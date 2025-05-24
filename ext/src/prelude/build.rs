use std::env::current_dir;
use std::fs::read_to_string;
use std::path::Path;

use airlang::AirCell;
use airlang::ConstRef;
use airlang::ConstStaticImpl;
use airlang::Contract;
use airlang::Ctx;
use airlang::FuncMode;
use airlang::FuncVal;
use airlang::PreludeCtx;
use airlang::Symbol;
use airlang::Text;
use airlang::Val;
use airlang::parse;

use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::named_const_fn;

pub(crate) struct BuildPrelude {
    pub(crate) import: Named<FuncVal>,
}

impl Default for BuildPrelude {
    fn default() -> Self {
        Self { import: import() }
    }
}

impl Prelude for BuildPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.import.put(ctx);
    }
}

fn import() -> Named<FuncVal> {
    let id = "build.import";
    let f = ConstStaticImpl::new(fn_import_free, fn_import_const);
    let mode = FuncMode::default();
    named_const_fn(id, f, mode)
}

const CUR_URL_KEY: &str = "build.this_url";

fn fn_import_free(input: Val) -> Val {
    let Val::Text(url) = input else {
        return Val::default();
    };
    let new_url = String::from(Text::from(url));
    import_from_url(new_url)
}

fn fn_import_const(mut ctx: ConstRef<Ctx>, input: Val) -> Val {
    let Val::Text(url) = input else {
        return Val::default();
    };
    let url = Text::from(url);
    let cur_url_key = unsafe { Symbol::from_str_unchecked(CUR_URL_KEY) };
    let cur_url = get_cur_url(ctx.reborrow(), cur_url_key);
    let new_url =
        cur_url.as_ref().and_then(|cur_url| join_url(cur_url, &url)).unwrap_or(String::from(url));
    import_from_url(new_url)
}

fn import_from_url(url: String) -> Val {
    let content = match read_to_string(&url) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("failed to read {url}: {err}");
            return Val::default();
        }
    };
    let Ok(val) = parse(&content) else {
        return Val::default();
    };

    let mut mod_air = AirCell::default();
    let cur_url_key = unsafe { Symbol::from_str_unchecked(CUR_URL_KEY) };
    if !set_cur_url(mod_air.ctx_mut(), cur_url_key, url) {
        return Val::default();
    }
    mod_air.interpret(val)
}

fn get_cur_url(ctx: ConstRef<Ctx>, key: Symbol) -> Option<String> {
    if let Ok(val) = ctx.get_ref(key) {
        return if let Val::Text(url) = val { Some((***url).clone()) } else { None };
    }
    let Ok(cur_dir) = current_dir() else {
        return None;
    };
    let Ok(cur_dir) = cur_dir.into_os_string().into_string() else {
        return None;
    };
    Some(cur_dir)
}

fn set_cur_url(ctx: &mut Ctx, key: Symbol, new_url: String) -> bool {
    ctx.put(key, Val::Text(Text::from(new_url).into()), Contract::None).is_ok()
}

fn join_url(cur_url: &str, url: &str) -> Option<String> {
    let parent = <_ as AsRef<Path>>::as_ref(cur_url).parent()?;
    let new_url = parent.join(url).canonicalize().ok()?;
    let new_url = new_url.into_os_string().into_string().ok()?;
    Some(new_url)
}
