use std::{
    env::current_dir,
    fs::read_to_string,
    path::Path,
    rc::Rc,
};

use airlang::{
    interpret_const,
    interpret_free,
    interpret_mutable,
    parse,
    Ctx,
    CtxForMutableFn,
    InvariantTag,
    MutableCtx,
    Str,
    Symbol,
    Val,
};

use crate::{
    prelude::{
        default_mode,
        put_func,
        Prelude,
    },
    ExtFn,
    ExtFunc,
};

pub(crate) struct BuildPrelude {
    pub(crate) import: Rc<ExtFunc>,
}

impl Default for BuildPrelude {
    fn default() -> Self {
        Self { import: import() }
    }
}

impl Prelude for BuildPrelude {
    fn put(&self, mut ctx: MutableCtx) {
        put_func(&self.import, ctx.reborrow());
    }
}

fn import() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("build.import") };
    let input_mode = default_mode();
    let output_mode = default_mode();
    let ext_fn = ExtFn::new_mutable(fn_import);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
}

const CUR_URL_KEY: &str = "build.this_url";

fn fn_import(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::String(url) = input else {
        return Val::default();
    };
    let cur_url_key = unsafe { Symbol::from_str_unchecked(CUR_URL_KEY) };
    let cur_url = get_cur_url(ctx.reborrow(), &cur_url_key);
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

    match ctx {
        CtxForMutableFn::Free(ctx) => interpret_free(ctx, val),
        CtxForMutableFn::Const(ctx) => interpret_const(ctx, val),
        CtxForMutableFn::Mutable(mut mutable_ctx) => {
            set_cur_url(mutable_ctx.reborrow(), cur_url_key.clone(), new_url);
            let output = interpret_mutable(mutable_ctx.reborrow(), val);
            if let Some(cur_url) = cur_url {
                set_cur_url(mutable_ctx, cur_url_key, cur_url);
            }
            output
        }
    }
}

fn get_cur_url(mut ctx: CtxForMutableFn, key: &Symbol) -> Option<String> {
    if let Ok(meta) = ctx.meta()
        && let Ok(val) = meta.get_ref(key)
    {
        if let Val::String(url) = val {
            Some((**url).to_owned())
        } else {
            None
        }
    } else {
        let Ok(cur_dir) = current_dir() else {
            return None;
        };
        let Ok(cur_dir) = cur_dir.into_os_string().into_string() else {
            return None;
        };
        Some(cur_dir)
    }
}

fn set_cur_url(mut ctx: MutableCtx, key: Symbol, new_url: String) {
    if let Some(mut meta) = ctx.meta() {
        let _ = meta.put(key, InvariantTag::None, Val::String(Str::from(new_url)));
    } else {
        let mut meta = Ctx::default();
        let mut meta_mut = MutableCtx::new(&mut meta);
        let _ = meta_mut.put(key, InvariantTag::None, Val::String(Str::from(new_url)));
        ctx.set_meta(Some(meta));
    }
}

fn join_url(cur_url: &str, url: &str) -> Option<String> {
    if let Some(parent) = <_ as AsRef<Path>>::as_ref(cur_url).parent()
        && let Ok(new_url) = parent.join(url).canonicalize()
        && let Ok(new_url) = new_url.into_os_string().into_string()
    {
        Some(new_url)
    } else {
        None
    }
}
