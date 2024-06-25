use std::{
    env::current_dir,
    fs::read_to_string,
    path::Path,
};

use airlang::{
    initial_ctx,
    interpret_mutable,
    parse,
    Ctx,
    CtxForMutableFn,
    FuncVal,
    Invariant,
    Mode,
    MutableCtx,
    Symbol,
    Text,
    Val,
};

use crate::{
    init_ctx,
    prelude::{
        named_mutable_fn,
        Named,
        Prelude,
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
    fn put(&self, mut ctx: MutableCtx) {
        self.import.put(ctx.reborrow());
    }
}

fn import() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mutable_fn("build.import", input_mode, output_mode, fn_import)
}

const CUR_URL_KEY: &str = "build.this_url";

fn fn_import(mut ctx: CtxForMutableFn, input: Val) -> Val {
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

    let mut ctx_for_mod = initial_ctx();
    let mut mut_ctx_for_mod = MutableCtx::new(&mut ctx_for_mod);
    init_ctx(mut_ctx_for_mod.reborrow());
    if !set_cur_url(mut_ctx_for_mod.reborrow(), cur_url_key, new_url) {
        return Val::default();
    }
    interpret_mutable(mut_ctx_for_mod, val)
}

fn get_cur_url(ctx: CtxForMutableFn, key: Symbol) -> Option<String> {
    if let Ok(meta) = ctx.meta() {
        if let Ok(val) = meta.get_ref(key) {
            return if let Val::Text(url) = val {
                Some((***url).clone())
            } else {
                None
            };
        }
    }
    let Ok(cur_dir) = current_dir() else {
        return None;
    };
    let Ok(cur_dir) = cur_dir.into_os_string().into_string() else {
        return None;
    };
    Some(cur_dir)
}

fn set_cur_url(mut ctx: MutableCtx, key: Symbol, new_url: String) -> bool {
    if let Some(meta) = ctx.reborrow().meta() {
        meta.put(key, Invariant::None, Val::Text(Text::from(new_url).into()))
            .is_ok()
    } else {
        let mut meta = Ctx::default();
        let meta_mut = MutableCtx::new(&mut meta);
        meta_mut
            .put(key, Invariant::None, Val::Text(Text::from(new_url).into()))
            .expect("put into a mutable empty ctx should never fail");
        ctx.set_meta(Some(meta));
        true
    }
}

fn join_url(cur_url: &str, url: &str) -> Option<String> {
    let parent = <_ as AsRef<Path>>::as_ref(cur_url).parent()?;
    let new_url = parent.join(url).canonicalize().ok()?;
    let new_url = new_url.into_os_string().into_string().ok()?;
    Some(new_url)
}
