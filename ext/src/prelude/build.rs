use std::{
    env::current_dir,
    fs::read_to_string,
    path::Path,
};

use airlang::{
    AirCell,
    ConstFnCtx,
    FuncMode,
    FuncVal,
    MutCtx,
    Symbol,
    Text,
    Val,
    VarAccess,
    parse,
};

use crate::{
    init_ctx,
    prelude::{
        Named,
        Prelude,
        named_const_fn,
    },
};

pub(crate) struct BuildPrelude {
    pub(crate) import: Named<FuncVal>,
}

impl Default for BuildPrelude {
    fn default() -> Self {
        Self { import: import() }
    }
}

impl Prelude for BuildPrelude {
    fn put(&self, mut ctx: MutCtx) {
        self.import.put(ctx.reborrow());
    }
}

fn import() -> Named<FuncVal> {
    let id = "build.import";
    let f = fn_import;
    let mode = FuncMode::default();
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

const CUR_URL_KEY: &str = "build.this_url";

fn fn_import(mut ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Text(url) = input else {
        return Val::default();
    };
    let url = Text::from(url);
    let cur_url_key = unsafe { Symbol::from_str_unchecked(CUR_URL_KEY) };
    let cur_url = get_cur_url(ctx.reborrow(), cur_url_key.clone());
    let new_url = cur_url
        .as_ref()
        .and_then(|cur_url| join_url(cur_url, &url))
        .unwrap_or(String::from(url));

    let content = match read_to_string(&new_url) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("failed to read {}: {}", new_url, err);
            return Val::default();
        }
    };
    let Ok(val) = parse(&content) else {
        return Val::default();
    };

    let mut mod_air = AirCell::default();
    init_ctx(mod_air.ctx_mut());
    if !set_cur_url(mod_air.ctx_mut(), cur_url_key, new_url) {
        return Val::default();
    }
    mod_air.interpret(val)
}

fn get_cur_url(ctx: ConstFnCtx, key: Symbol) -> Option<String> {
    if let Ok(val) = ctx.get_ref(key) {
        return if let Val::Text(url) = val {
            Some((***url).clone())
        } else {
            None
        };
    }
    let Ok(cur_dir) = current_dir() else {
        return None;
    };
    let Ok(cur_dir) = cur_dir.into_os_string().into_string() else {
        return None;
    };
    Some(cur_dir)
}

fn set_cur_url(ctx: MutCtx, key: Symbol, new_url: String) -> bool {
    ctx.put(
        key,
        VarAccess::Assign,
        Val::Text(Text::from(new_url).into()),
    )
    .is_ok()
}

fn join_url(cur_url: &str, url: &str) -> Option<String> {
    let parent = <_ as AsRef<Path>>::as_ref(cur_url).parent()?;
    let new_url = parent.join(url).canonicalize().ok()?;
    let new_url = new_url.into_os_string().into_string().ok()?;
    Some(new_url)
}
